//! Reading exercises (expansion M7, ADR 0035): the sets, the answer check and progress.
//! Answers never reach the UI before the user has answered; the check is here, pure.

use kb_core::Language;
use kb_store::reading::{ReadingItemData, ReadingKind, normalize_output};
use kb_store::reading_answers::ReadingAnswerRow;
use kb_store::{ReadingLink, ReadingSetSummary, ReadingSetView, UiLang};
use serde::{Deserialize, Serialize};

use crate::{App, AppError, AppErrorCode, Result};

/// What the user answered: an output (print items), or time and space (complexity items).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingAnswer {
    /// The version the user read: `python` or `typescript`.
    pub language: Language,
    pub output: Option<String>,
    pub time: Option<String>,
    pub space: Option<String>,
}

/// The verdict, with everything the feedback shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ReadingResult {
    pub correct: bool,
    /// Print items: the exact output of the version read.
    pub output: Option<String>,
    pub time: Option<String>,
    pub space: Option<String>,
    /// Complexity items: the line that dominates the cost in the version read (1-based).
    pub dominant_line: Option<u32>,
    pub explanation: String,
    pub attempts: u32,
}

/// Whether `answer` is right for `item` (pure).
pub fn check_answer(kind: ReadingKind, item: &ReadingItemData, answer: &ReadingAnswer) -> bool {
    let ts = answer.language == Language::TypeScript;
    match kind {
        ReadingKind::Print => match (item.answer_for(ts), &answer.output) {
            (Some(expected), Some(given)) => normalize_output(expected) == normalize_output(given),
            _ => false,
        },
        ReadingKind::Complexity => {
            let same = |a: &Option<String>, b: &Option<String>| a.is_some() && a == b;
            same(&item.time, &answer.time) && same(&item.space, &answer.space)
        }
    }
}

fn bad(msg: impl Into<String>) -> AppError {
    AppError::new(AppErrorCode::InvalidInput, msg.into())
}

impl App {
    pub async fn reading_sets(&self, lang: UiLang) -> Result<Vec<ReadingSetSummary>> {
        Ok(self.store.content.reading_sets(lang).await?)
    }

    pub async fn reading_set(&self, id: &str, lang: UiLang) -> Result<ReadingSetView> {
        Ok(self.store.content.reading_set(id, lang).await?)
    }

    /// Progress: every answered item.
    pub async fn reading_answers(&self) -> Result<Vec<ReadingAnswerRow>> {
        Ok(self.store.user.reading_answers().await?)
    }

    pub async fn reading_links(&self, topic: &str, lang: UiLang) -> Result<Vec<ReadingLink>> {
        Ok(self.store.content.reading_links(topic, lang).await?)
    }

    /// Check an answer, count the attempt, and reveal the answer and explanation.
    pub async fn answer_reading(
        &self,
        item: &str,
        answer: ReadingAnswer,
        lang: UiLang,
    ) -> Result<ReadingResult> {
        if !matches!(answer.language, Language::Python | Language::TypeScript) {
            return Err(bad("reading items come in Python and TypeScript"));
        }
        if answer.output.as_ref().is_some_and(|o| o.len() > 4096) {
            return Err(bad("answer too long"));
        }
        let full = self.store.content.reading_item(item).await?;
        let correct = check_answer(full.kind, &full.data, &answer);
        let row = self
            .store
            .user
            .record_reading_answer(item, correct, self.now_ms())
            .await?;
        let ts = answer.language == Language::TypeScript;
        let d = full.data;
        Ok(ReadingResult {
            correct,
            output: d.answer_for(ts).map(str::to_string),
            dominant_line: d.dominant.map(|l| if ts { l.typescript } else { l.python }),
            explanation: if lang == UiLang::Es && !d.explain_es.is_empty() {
                d.explain_es
            } else {
                d.explain
            },
            time: d.time,
            space: d.space,
            attempts: row.attempts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_item() -> ReadingItemData {
        ReadingItemData {
            n: 1,
            prompt: "p".into(),
            prompt_es: "p".into(),
            explain: "e".into(),
            explain_es: "e".into(),
            answer: None,
            answer_py: Some("3.5".into()),
            answer_ts: Some("3.5\n".into()),
            choices: vec![],
            time: None,
            space: None,
            dominant: None,
            sizes: vec![],
        }
    }

    fn answer(language: Language, output: &str) -> ReadingAnswer {
        ReadingAnswer {
            language,
            output: Some(output.into()),
            time: None,
            space: None,
        }
    }

    #[test]
    fn print_answers_compare_line_by_line_per_language() {
        let item = print_item();
        assert!(check_answer(
            ReadingKind::Print,
            &item,
            &answer(Language::Python, "3.5  \n\n")
        ));
        assert!(check_answer(
            ReadingKind::Print,
            &item,
            &answer(Language::TypeScript, "3.5")
        ));
        assert!(!check_answer(
            ReadingKind::Print,
            &item,
            &answer(Language::Python, "3")
        ));
        assert!(!check_answer(
            ReadingKind::Print,
            &item,
            &answer(Language::Python, " 3.5")
        ));
    }

    #[test]
    fn complexity_needs_both_time_and_space() {
        let mut item = print_item();
        item.time = Some("O(n)".into());
        item.space = Some("O(1)".into());
        let a = |t: &str, s: &str| ReadingAnswer {
            language: Language::Python,
            output: None,
            time: Some(t.into()),
            space: Some(s.into()),
        };
        assert!(check_answer(
            ReadingKind::Complexity,
            &item,
            &a("O(n)", "O(1)")
        ));
        assert!(!check_answer(
            ReadingKind::Complexity,
            &item,
            &a("O(n)", "O(n)")
        ));
        assert!(!check_answer(
            ReadingKind::Complexity,
            &item,
            &a("O(n²)", "O(1)")
        ));
    }
}
