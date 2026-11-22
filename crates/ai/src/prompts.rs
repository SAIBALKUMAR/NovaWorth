//! Canonical prompt templates and task profile.

use common::constants::{
    AI_MAX_TOKENS_DRAFT, AI_MAX_TOKENS_EXPLANATION, AI_MAX_TOKENS_FEEDBACK,
    OPENAI_MODEL_DEFAULT,
};

/// The kinds of LLM work the gateway supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTask {
    Explanation,
    SocraticQuestion,
    FeedbackOnAnswer,
    ContentDraft,
    Embedding,
}

impl AiTask {
    pub fn model(self) -> &'static str {
        match self {
            // Embeddings use a distinct model handled by the embeddings module.
            AiTask::Embedding => "text-embedding-3-small",
            _ => OPENAI_MODEL_DEFAULT,
        }
    }

    pub fn max_tokens(self) -> u32 {
        match self {
            AiTask::Explanation => AI_MAX_TOKENS_EXPLANATION,
            AiTask::SocraticQuestion => 256,
            AiTask::FeedbackOnAnswer => AI_MAX_TOKENS_FEEDBACK,
            AiTask::ContentDraft => AI_MAX_TOKENS_DRAFT,
            AiTask::Embedding => 0,
        }
    }

    pub fn temperature(self) -> f32 {
        match self {
            AiTask::Explanation => 0.3,
            AiTask::SocraticQuestion => 0.7,
            AiTask::FeedbackOnAnswer => 0.4,
            AiTask::ContentDraft => 0.5,
            AiTask::Embedding => 0.0,
        }
    }
}

/// Constant system prompt beacons.
pub const SOCRATIC_SYSTEM: &str =
    "You are a patient Socratic tutor. Never reveal the answer; ask one guiding question at a time.";
pub const FEEDBACK_SYSTEM: &str =
    "You give concise, rubric-aligned feedback on a learner's answer. Be specific and kind.";
pub const DRAFT_SYSTEM: &str =
    "You draft curriculum content. Mark all AI-generated output and cite sources where relevant.";

/// Constant system prompt for concept explanations.
pub const EXPLANATION_SYSTEM: &str =
    "You explain concepts clearly and accurately, at the learner's level. No fluff, no unsupported claims.";

/// Build a Socratic question prompt for a concept within a level.
pub fn socratic_question_prompt(level_title: &str, concept: &str) -> String {
    format!(
        "Level: {level_title}\nConcept being learned: {concept}\nAsk the learner one Socratic question to probe their understanding."
    )
}

/// Build a feedback prompt for a learner's answer against a rubric.
pub fn feedback_prompt(answer: &str, rubric: &str) -> String {
    format!(
        "Learner answer:\n{answer}\n\nRubric:\n{rubric}\n\nGive constructive, criterion-by-criterion feedback."
    )
}

/// Build an explanation prompt for a topic.
pub fn explanation_prompt(topic: &str) -> String {
    format!("Explain the following concept clearly and accurately:\n{topic}")
}

/// Build a content draft prompt for a skill level.
pub fn content_draft_prompt(skill: &str, level: u32, objectives: &str) -> String {
    format!(
        "Draft level {level} learning content for the skill '{skill}'.\nLearning objectives:\n{objectives}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_profiles_are_distinct() {
        assert_eq!(AiTask::Embedding.max_tokens(), 0);
        assert!(AiTask::ContentDraft.max_tokens() > AiTask::Explanation.max_tokens());
    }

    #[test]
    fn prompts_embed_inputs() {
        assert!(socratic_question_prompt("Intro", "variables").contains("variables"));
        assert!(feedback_prompt("2+2=4", "clarity").contains("2+2=4"));
        assert!(content_draft_prompt("Cooking", 12, "knife skills").contains("knife skills"));
    }
}