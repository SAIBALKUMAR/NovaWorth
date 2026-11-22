//! Order placement with atomic inventory reservation and balance pre-check.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// A single line of an order.
#[derive(Debug, Clone, Copy)]
pub struct OrderLine {
    pub product_id: Uuid,
    pub quantity: i32,
}

/// Creates orders and reserves stock atomically.
pub struct OrderService {
    pool: PgPool,
}

impl OrderService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Validate products, reserve stock, pre-check EC balance, and create an order
    /// plus its line items — all in one transaction.
    ///
    /// Returns the new `order_id` (status `pending`), ready to be escrowed.
    pub async fn place_order(&self, user_id: Uuid, lines: &[OrderLine]) -> Result<Uuid, AppError> {
        if lines.is_empty() {
            return Err(AppError::Validation(
                "Order must contain at least one item".into(),
            ));
        }

        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        let mut ec_total: i64 = 0;
        let mut items: Vec<(Uuid, i32, i64)> = Vec::with_capacity(lines.len()); // (product, qty, price)

        for line in lines {
            if line.quantity <= 0 {
                return Err(AppError::Validation(
                    "Item quantity must be positive".into(),
                ));
            }

            let product: Option<(bool, i32, i64)> = sqlx::query_as(
                "SELECT active, stock, price_ec FROM products WHERE id = $1 FOR UPDATE",
            )
            .bind(line.product_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            let (active, stock, price_ec) = product
                .ok_or_else(|| AppError::NotFound { resource: "Product".into() })?;

            if !active {
                return Err(AppError::Validation(
                    "That item is no longer available".into(),
                ));
            }
            if stock < line.quantity {
                return Err(AppError::Validation(format!(
                    "Only {stock} in stock (you asked for {})",
                    line.quantity
                )));
            }

            // Atomic decrement guarded by `stock >= quantity` as a concurrency fence.
            sqlx::query("UPDATE products SET stock = stock - $2 WHERE id = $1 AND stock >= $2")
                .bind(line.product_id)
                .bind(line.quantity)
                .execute(&mut *tx)
                .await
                .map_err(AppError::Database)?;

            ec_total += price_ec * line.quantity as i64;
            items.push((line.product_id, line.quantity, price_ec));
        }

        // Fast-fail on insufficient EC before committing the order, so the learner
        // is never left with a payable-but-unfunded order.
        sqlx::query("INSERT INTO wallets (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING")
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        let row: (i64,) =
            sqlx::query_as("SELECT ec_balance FROM wallets WHERE user_id = $1 FOR UPDATE")
                .bind(user_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(AppError::Database)?;

        if row.0 < ec_total {
            return Err(AppError::InsufficientFunds {
                need: ec_total,
                have: row.0,
                currency: "EC".into(),
            });
        }

        let order_row: (Uuid,) = sqlx::query_as(
            "INSERT INTO orders (user_id, status, ec_total)
             VALUES ($1, 'pending', $2)
             RETURNING id",
        )
        .bind(user_id)
        .bind(ec_total)
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::Database)?;
        let order_id = order_row.0;

        for (product_id, quantity, price_ec) in items {
            sqlx::query(
                "INSERT INTO order_items (order_id, product_id, quantity, price_ec)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(order_id)
            .bind(product_id)
            .bind(quantity)
            .bind(price_ec)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(order_id)
    }
}