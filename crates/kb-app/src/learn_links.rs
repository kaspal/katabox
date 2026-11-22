//! Learn ↔ practice links (features-content M1). A Learn topic lists its catalogue
//! problems (ADR 0029). From that one link come the pattern pages' "Learn: …" topics
//! (the topics linked to a problem of that pattern) and Home's "Review a topic" card (a
//! viewed topic whose linked problems are not all solved). No new content data.

use std::collections::{BTreeMap, HashMap};

use kb_store::{LearnTopicSummary, ProblemFilter, ProblemStatus, ProblemSummary, UiLang};
use serde::Serialize;

use crate::{App, Result};

/// The Learn topics for one practice pattern, in Learn's own order.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PatternTopics {
    pub pattern: String,
    pub topics: Vec<LearnTopicSummary>,
}

/// Home's "Review a topic": a topic the user watched, and a linked problem not solved yet.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TopicReview {
    pub topic: LearnTopicSummary,
    pub problem: ProblemSummary,
}

/// Pattern → topics, from each topic's linked problems.
pub fn topics_by_pattern(
    topics: &[LearnTopicSummary],
    problems: &[ProblemSummary],
) -> Vec<PatternTopics> {
    let pattern_of: HashMap<&str, &str> = problems
        .iter()
        .map(|p| (p.id.as_str(), p.pattern.as_str()))
        .collect();
    let mut by: BTreeMap<&str, Vec<LearnTopicSummary>> = BTreeMap::new();
    for t in topics {
        let mut patterns: Vec<&str> = t
            .problems
            .iter()
            .filter_map(|id| pattern_of.get(id.as_str()).copied())
            .collect();
        patterns.sort_unstable();
        patterns.dedup();
        for p in patterns {
            by.entry(p).or_default().push(t.clone());
        }
    }
    by.into_iter()
        .map(|(pattern, topics)| PatternTopics {
            pattern: pattern.to_string(),
            topics,
        })
        .collect()
}

/// The most recently viewed topic (last in `viewed`) that links an unsolved problem, with
/// its first unsolved problem by catalogue rank.
pub fn topic_to_review(
    viewed: &[String],
    topics: &[LearnTopicSummary],
    problems: &[ProblemSummary],
) -> Option<TopicReview> {
    let by_id: HashMap<&str, &ProblemSummary> =
        problems.iter().map(|p| (p.id.as_str(), p)).collect();
    viewed.iter().rev().find_map(|id| {
        let topic = topics.iter().find(|t| &t.id == id)?;
        let problem = topic
            .problems
            .iter()
            .filter_map(|p| by_id.get(p.as_str()).copied())
            .filter(|p| p.status != Some(ProblemStatus::Solved))
            .min_by_key(|p| (p.rank, p.id.clone()))?;
        Some(TopicReview {
            topic: topic.clone(),
            problem: problem.clone(),
        })
    })
}

impl App {
    async fn learn_link_data(
        &self,
        lang: UiLang,
    ) -> Result<(Vec<LearnTopicSummary>, Vec<ProblemSummary>)> {
        let topics = self
            .store
            .content
            .learn_catalog(lang)
            .await?
            .into_iter()
            .flat_map(|s| s.topics)
            .collect();
        let problems = self
            .store
            .list_problems(&ProblemFilter {
                lang: Some(lang),
                ..Default::default()
            })
            .await?;
        Ok((topics, problems))
    }

    /// The Learn topics for each practice pattern (Library and Progress link to them).
    pub async fn learn_by_pattern(&self, lang: UiLang) -> Result<Vec<PatternTopics>> {
        let (topics, problems) = self.learn_link_data(lang).await?;
        Ok(topics_by_pattern(&topics, &problems))
    }

    /// Home's "Review a topic" card: `viewed` is the `learn.viewed` setting, in order.
    pub async fn topic_to_review(
        &self,
        viewed: &[String],
        lang: UiLang,
    ) -> Result<Option<TopicReview>> {
        let (topics, problems) = self.learn_link_data(lang).await?;
        Ok(topic_to_review(viewed, &topics, &problems))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::{Difficulty, ProblemMode};

    fn topic(id: &str, problems: &[&str]) -> LearnTopicSummary {
        LearnTopicSummary {
            id: id.into(),
            subject: "s".into(),
            title: id.into(),
            tags: vec![],
            problems: problems.iter().map(|p| p.to_string()).collect(),
        }
    }

    fn problem(id: &str, pattern: &str, rank: u8, status: Option<ProblemStatus>) -> ProblemSummary {
        ProblemSummary {
            id: id.into(),
            title: id.into(),
            difficulty: Difficulty::Easy,
            pattern: pattern.into(),
            rank,
            mode: ProblemMode::Function,
            tags: vec![],
            status,
            fix: false,
        }
    }

    #[test]
    fn a_pattern_gets_the_topics_linked_to_its_problems() {
        let topics = [
            topic("bfs", &["maze", "rooms"]),
            topic("heap", &["tasks"]),
            topic("none", &[]),
        ];
        let problems = [
            problem("maze", "graphs", 1, None),
            problem("rooms", "graphs", 2, None),
            problem("tasks", "heap", 1, None),
        ];
        let by = topics_by_pattern(&topics, &problems);
        let names: Vec<(&str, Vec<&str>)> = by
            .iter()
            .map(|p| {
                (
                    p.pattern.as_str(),
                    p.topics.iter().map(|t| t.id.as_str()).collect(),
                )
            })
            .collect();
        // bfs links two graph problems but is listed once.
        assert_eq!(names, [("graphs", vec!["bfs"]), ("heap", vec!["heap"])]);
    }

    #[test]
    fn review_picks_the_latest_viewed_topic_with_an_unsolved_problem() {
        let topics = [topic("bfs", &["maze", "rooms"]), topic("heap", &["tasks"])];
        let problems = [
            problem("maze", "graphs", 2, None),
            problem("rooms", "graphs", 1, Some(ProblemStatus::Attempted)),
            problem("tasks", "heap", 1, Some(ProblemStatus::Solved)),
        ];
        let viewed = ["bfs".to_string(), "heap".to_string()];
        // heap was viewed last, but its only problem is solved: bfs, lowest rank unsolved.
        let r = topic_to_review(&viewed, &topics, &problems).unwrap();
        assert_eq!(
            (r.topic.id.as_str(), r.problem.id.as_str()),
            ("bfs", "rooms")
        );
        let all_solved = [
            problem("maze", "graphs", 2, Some(ProblemStatus::Solved)),
            problem("rooms", "graphs", 1, Some(ProblemStatus::Solved)),
        ];
        assert_eq!(topic_to_review(&viewed, &topics, &all_solved), None);
        assert_eq!(topic_to_review(&[], &topics, &problems), None);
    }
}
