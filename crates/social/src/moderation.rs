//! Moderation heuristics (content classification).
//!
//! A dependency-free first line of defense; escalate high-severity hits to human
//! moderators. The reports table can be added behind these pure functions later.

/// Classification outcome for a piece of user-authored text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModerationDecision {
    pub abusive: bool,
    /// 0 = clean, 1 = mild, 2 = severe, 3 = critical.
    pub severity: u8,
}

const ABUSE_TERMS: &[&str] = &[
    "hate speech",
    "violence",
    "self-harm",
    "dox",
    "harass",
    "slur",
];

const CRITICAL_TERMS: &[&str] = &["self-harm", "dox", "slur"];

/// Classify text into a moderation decision.
pub fn classify(text: &str) -> ModerationDecision {
    let lower = text.to_lowercase();
    let critical = CRITICAL_TERMS.iter().filter(|t| lower.contains(**t)).count();
    let total = ABUSE_TERMS.iter().filter(|t| lower.contains(**t)).count();

    let severity = if critical > 0 {
        3
    } else if total >= 2 {
        2
    } else if total == 1 {
        1
    } else {
        0
    };

    ModerationDecision {
        abusive: total > 0,
        severity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_is_allowed() {
        let d = classify("I learned a lot from this lesson");
        assert!(!d.abusive);
        assert_eq!(d.severity, 0);
    }

    #[test]
    fn critical_terms_escalate() {
        let d = classify("please dox that person");
        assert!(d.abusive);
        assert_eq!(d.severity, 3);
    }

    #[test]
    fn multiple_terms_raise_severity() {
        let d = classify("harass and hate speech");
        assert!(d.abusive);
        assert_eq!(d.severity, 2);
    }
}