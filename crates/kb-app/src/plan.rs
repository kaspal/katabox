//! Study plans (expansion M2, ADR 0033): "a goal, a number of weeks and minutes a day"
//! becomes a day-by-day schedule of Learn topics, problems, reviews and katas, with
//! done / skip / reschedule and an `.ics` calendar export. The generator and the ICS
//! writer are pure and unit tested.

use kb_core::{Day, Difficulty};
use serde::{Deserialize, Serialize};

/// Minutes each kind of item is planned for.
pub const EASY_MIN: u32 = 20;
pub const MEDIUM_MIN: u32 = 35;
pub const HARD_MIN: u32 = 50;
pub const TOPIC_MIN: u32 = 15;
pub const REVIEW_MIN: u32 = 10;
pub const KATA_MIN: u32 = 15;
/// Plans run 1 to 26 weeks, 10 to 240 minutes a day.
pub const WEEKS_MAX: u32 = 26;
pub const MINUTES_MIN: u32 = 10;
pub const MINUTES_MAX: u32 = 240;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum PlanItemKind {
    Problem,
    Topic,
    Review,
    Kata,
}

impl PlanItemKind {
    pub fn id(self) -> &'static str {
        match self {
            PlanItemKind::Problem => "problem",
            PlanItemKind::Topic => "topic",
            PlanItemKind::Review => "review",
            PlanItemKind::Kata => "kata",
        }
    }

    pub fn parse(s: &str) -> Option<PlanItemKind> {
        Some(match s {
            "problem" => PlanItemKind::Problem,
            "topic" => PlanItemKind::Topic,
            "review" => PlanItemKind::Review,
            "kata" => PlanItemKind::Kata,
            _ => return None,
        })
    }
}

/// What the generator schedules for one pattern, in order: its unviewed Learn topics,
/// then its unsolved problems by rank.
#[derive(Debug, Clone, PartialEq)]
pub struct PatternWork {
    pub pattern: String,
    pub topics: Vec<String>,
    pub problems: Vec<(String, Difficulty)>,
}

/// One planned item (before it is stored).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Planned {
    pub day: Day,
    pub kind: PlanItemKind,
    /// The problem or topic id (none for reviews and katas).
    pub reference: Option<String>,
    pub minutes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generated {
    pub items: Vec<Planned>,
    /// Topics and problems that did not fit the plan's days.
    pub left_over: u32,
}

pub fn problem_minutes(d: Difficulty) -> u32 {
    match d {
        Difficulty::Easy => EASY_MIN,
        Difficulty::Medium => MEDIUM_MIN,
        Difficulty::Hard => HARD_MIN,
    }
}

/// Fill `weeks × 7` days from `start`: each day up to `minutes_per_day`; the last day of
/// every week opens with a kata and the due reviews. An item longer than a whole day
/// gets a day of its own. Work never moves backwards: the order of `work` is kept.
pub fn generate(work: &[PatternWork], start: Day, weeks: u32, minutes_per_day: u32) -> Generated {
    let mut queue: Vec<(PlanItemKind, String, u32)> = Vec::new();
    for w in work {
        for t in &w.topics {
            queue.push((PlanItemKind::Topic, t.clone(), TOPIC_MIN));
        }
        for (p, d) in &w.problems {
            queue.push((PlanItemKind::Problem, p.clone(), problem_minutes(*d)));
        }
    }
    let mut items = Vec::new();
    let mut next = 0usize;
    for d in 0..i64::from(weeks) * 7 {
        let day = start.add_days(d);
        let mut left = minutes_per_day;
        if d % 7 == 6 {
            for (kind, min) in [
                (PlanItemKind::Kata, KATA_MIN),
                (PlanItemKind::Review, REVIEW_MIN),
            ] {
                if min <= left {
                    items.push(Planned {
                        day,
                        kind,
                        reference: None,
                        minutes: min,
                    });
                    left -= min;
                }
            }
        }
        let fresh = left == minutes_per_day;
        while next < queue.len() {
            let (kind, id, min) = &queue[next];
            if *min > left && !(fresh && left == minutes_per_day) {
                break;
            }
            items.push(Planned {
                day,
                kind: *kind,
                reference: Some(id.clone()),
                minutes: *min,
            });
            left = left.saturating_sub(*min);
            next += 1;
            if left == 0 {
                break;
            }
        }
    }
    Generated {
        items,
        left_over: u32::try_from(queue.len() - next).unwrap_or(u32::MAX),
    }
}

// --- ICS (RFC 5545) -------------------------------------------------------------------

/// Escape a TEXT value: backslash, semicolon, comma and newlines.
pub fn ics_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            ';' => out.push_str("\\;"),
            ',' => out.push_str("\\,"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            c => out.push(c),
        }
    }
    out
}

/// Fold a content line at 75 octets (never inside a UTF-8 character), CRLF-terminated.
pub fn ics_fold(line: &str) -> String {
    let mut out = String::new();
    let mut used = 0usize;
    let mut limit = 75usize;
    for c in line.chars() {
        let n = c.len_utf8();
        if used + n > limit {
            out.push_str("\r\n ");
            used = 0;
            // Continuation lines start with a space, which counts.
            limit = 74;
        }
        out.push(c);
        used += n;
    }
    out.push_str("\r\n");
    out
}

/// One calendar day of a plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IcsDay {
    pub day: Day,
    /// The day's items, already worded in the UI language.
    pub lines: Vec<String>,
    pub minutes: u32,
}

fn ics_date(d: Day) -> String {
    d.to_string().replace('-', "")
}

/// A calendar with one all-day event per plan day. `stamp` is the export time as
/// `YYYYMMDDTHHMMSSZ`; `uid_base` makes the event ids unique per plan.
pub fn plan_ics(title: &str, uid_base: &str, stamp: &str, days: &[IcsDay]) -> String {
    let mut s = String::new();
    for l in [
        "BEGIN:VCALENDAR",
        "VERSION:2.0",
        "PRODID:-//KataBox//Study plan//EN",
        "CALSCALE:GREGORIAN",
    ] {
        s.push_str(&ics_fold(l));
    }
    for d in days {
        s.push_str(&ics_fold("BEGIN:VEVENT"));
        s.push_str(&ics_fold(&format!(
            "UID:{uid_base}-{}@katabox.local",
            ics_date(d.day)
        )));
        s.push_str(&ics_fold(&format!("DTSTAMP:{stamp}")));
        s.push_str(&ics_fold(&format!(
            "DTSTART;VALUE=DATE:{}",
            ics_date(d.day)
        )));
        s.push_str(&ics_fold(&format!(
            "DTEND;VALUE=DATE:{}",
            ics_date(d.day.add_days(1))
        )));
        s.push_str(&ics_fold(&format!(
            "SUMMARY:{}",
            ics_escape(&format!("{title} ({} min)", d.minutes))
        )));
        s.push_str(&ics_fold(&format!(
            "DESCRIPTION:{}",
            ics_escape(&d.lines.join("\n"))
        )));
        s.push_str(&ics_fold("TRANSP:TRANSPARENT"));
        s.push_str(&ics_fold("END:VEVENT"));
    }
    s.push_str(&ics_fold("END:VCALENDAR"));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(s: &str) -> Day {
        s.parse().unwrap()
    }

    fn work() -> Vec<PatternWork> {
        vec![
            PatternWork {
                pattern: "arrays-hashing".into(),
                topics: vec!["hash-tables".into()],
                problems: vec![
                    ("two-sum".into(), Difficulty::Easy),
                    ("budget-pair".into(), Difficulty::Easy),
                    ("balanced-streaks".into(), Difficulty::Medium),
                ],
            },
            PatternWork {
                pattern: "graphs".into(),
                topics: vec![],
                problems: vec![("cheapest-route".into(), Difficulty::Hard)],
            },
        ]
    }

    #[test]
    fn fills_days_in_order_within_the_budget() {
        let g = generate(&work(), day("2026-11-23"), 1, 40);
        let by_day = |d: &str| -> Vec<(PlanItemKind, Option<String>)> {
            g.items
                .iter()
                .filter(|i| i.day == day(d))
                .map(|i| (i.kind, i.reference.clone()))
                .collect()
        };
        // 15 + 20 fit in 40; the next easy (20) does not.
        assert_eq!(
            by_day("2026-11-23"),
            [
                (PlanItemKind::Topic, Some("hash-tables".into())),
                (PlanItemKind::Problem, Some("two-sum".into()))
            ]
        );
        assert_eq!(
            by_day("2026-11-24"),
            [(PlanItemKind::Problem, Some("budget-pair".into()))]
        );
        assert_eq!(
            by_day("2026-11-25"),
            [(PlanItemKind::Problem, Some("balanced-streaks".into()))]
        );
        // A hard (50) is longer than a whole day: it gets a day of its own.
        assert_eq!(
            by_day("2026-11-26"),
            [(PlanItemKind::Problem, Some("cheapest-route".into()))]
        );
        // The week's last day: a kata and the reviews.
        assert_eq!(
            by_day("2026-11-29"),
            [(PlanItemKind::Kata, None), (PlanItemKind::Review, None)]
        );
        assert_eq!(g.left_over, 0);
        for d in 0..7 {
            let total: u32 = g
                .items
                .iter()
                .filter(|i| i.day == day("2026-11-23").add_days(d))
                .map(|i| i.minutes)
                .sum();
            assert!(total <= 50, "day {d}: {total}");
        }
    }

    #[test]
    fn says_what_did_not_fit() {
        let g = generate(&work(), day("2026-11-23"), 1, 10);
        // Ten minutes a day: each item gets its own day; the kata (15) does not fit the
        // last day and the review (10) does.
        assert_eq!(g.left_over, 0);
        let g = generate(&work()[..1], day("2026-11-23"), 0, 60);
        assert!(g.items.is_empty());
        assert_eq!(g.left_over, 4);
    }

    #[test]
    fn ics_escapes_folds_and_uses_crlf() {
        assert_eq!(ics_escape("a,b;c\\d\ne"), "a\\,b\\;c\\\\d\\ne");
        let long = format!("DESCRIPTION:{}", "é".repeat(60));
        let folded = ics_fold(&long);
        for (i, line) in folded.split("\r\n").filter(|l| !l.is_empty()).enumerate() {
            assert!(line.len() <= 75, "{i}: {}", line.len());
            if i > 0 {
                assert!(line.starts_with(' '));
            }
        }
        assert_eq!(folded.replace("\r\n ", ""), format!("{long}\r\n"));
        let cal = plan_ics(
            "KataBox plan",
            "plan-1",
            "20261121T120000Z",
            &[IcsDay {
                day: day("2026-11-30"),
                lines: vec!["Two Sum, easy".into(), "Topic: hash tables".into()],
                minutes: 35,
            }],
        );
        assert!(cal.starts_with("BEGIN:VCALENDAR\r\n"));
        assert!(cal.ends_with("END:VCALENDAR\r\n"));
        assert!(cal.contains("DTSTART;VALUE=DATE:20261130\r\n"));
        assert!(cal.contains("DTEND;VALUE=DATE:20261201\r\n"));
        assert!(cal.contains("UID:plan-1-20261130@katabox.local\r\n"));
        assert!(cal.contains("SUMMARY:KataBox plan (35 min)\r\n"));
        assert!(cal.contains("DESCRIPTION:Two Sum\\, easy\\nTopic: hash tables\r\n"));
        assert!(!cal.replace("\r\n", "").contains('\n'));
    }
}
