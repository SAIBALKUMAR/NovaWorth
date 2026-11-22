//! Shared domain type aliases used across all services.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type UserId   = Uuid;
pub type LevelId  = Uuid;
pub type CourseId = Uuid;
pub type OrderId  = Uuid;
pub type GuildId  = Uuid;

/// Currency denomination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum Currency {
    XP,
    EC,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::XP => write!(f, "XP"),
            Currency::EC => write!(f, "EC"),
        }
    }
}

/// Ledger entry direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum EntryType {
    #[sqlx(rename = "CREDIT")]
    Credit,
    #[sqlx(rename = "DEBIT")]
    Debit,
}

/// User role in the platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum UserRole {
    #[sqlx(rename = "learner")]
    Learner,
    #[sqlx(rename = "mentor")]
    Mentor,
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "sponsor")]
    Sponsor,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Learner  => write!(f, "learner"),
            UserRole::Mentor   => write!(f, "mentor"),
            UserRole::Admin    => write!(f, "admin"),
            UserRole::Sponsor  => write!(f, "sponsor"),
        }
    }
}

/// Pagination params for list endpoints
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page:     Option<u32>,
    pub per_page: Option<u32>,
}

impl Pagination {
    pub fn offset(&self) -> u32 {
        let page = self.page.unwrap_or(1).max(1);
        let per  = self.per_page();
        (page - 1) * per
    }

    pub fn per_page(&self) -> u32 {
        self.per_page
            .unwrap_or(crate::constants::DEFAULT_PAGE_SIZE)
            .min(crate::constants::MAX_PAGE_SIZE)
    }
}

/// Standard API list response envelope
#[derive(Debug, Serialize)]
pub struct ListResponse<T: Serialize> {
    pub data:        Vec<T>,
    pub total:       i64,
    pub page:        u32,
    pub per_page:    u32,
    pub total_pages: u32,
}

impl<T: Serialize> ListResponse<T> {
    pub fn new(data: Vec<T>, total: i64, pagination: &Pagination) -> Self {
        let per_page = pagination.per_page();
        let page     = pagination.page.unwrap_or(1);
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
        Self { data, total, page, per_page, total_pages }
    }
}
