# 34. Mistake journal, reflections, snippets and search

Date: 2026-11-21 · Status: proposed (expansion M3-M4)

## Context
- Failed submissions were kept with their full report, but nothing summarised them.
- Users couldn't record why an accepted solution works, to read again later.

## Decision: mistake journal (M3)
**Data** (user.db migration 0007):
- `mistakes`: one row per failed Submit. It is keyed by the submission's natural key
  (problem, language, `created_at`) so it merges with the same submission on import.
  It holds `auto_tags`, the user's `tags` (starting as the automatic ones) and a note.
- `reflections`: one row per problem, with the "why it works" sentence (at most 500
  characters) and optional time and space classes from a fixed list.

**Automatic tags** (pure, `kb-app/src/journal.rs`, from the stored unredacted report):

| Kind | Tags |
| --- | --- |
| Verdict | `wa`, `tle`, `mle`, `ole`, `compile` |
| Runtime error | `runtime:index`, `runtime:key`, `runtime:null`, `runtime:zero_division`, `runtime:recursion`, `runtime:overflow`, `runtime:type`, `runtime:other` |
| Edge case | `edge:empty`, `edge:single`, `edge:negative`, `edge:zero`, `edge:duplicates`, `edge:max` |
| Shrinker | `shrunk` |
| Users only | `logic` |

- **Runtime errors:** the kind is read from the failure's message and stderr, with the
  Python, TypeScript, PHP and Rust wordings.
- **Edge cases** come from the failing input checked against the signature. They are
  for function problems only; program inputs are free text.
  - `edge:max` means an int equal to the largest magnitude, or an array or string length
    equal to the largest one, that parameter takes across the problem's tests.
- **Shrinker:** `shrunk` is added when the shrinker finds a smaller failing input for the
  failure of the last Submit.

**When tagging happens:**
- after each failed Submit;
- lazily, for failures that predate the journal (on the first look at the journal).

**Users** pick from the same fixed set (validated in kb-app) and write a note of at most
2000 characters.

**Locks:**
- **Mock interviews:** the journal follows the interview lock (`ensure_unlocked`), and so
  does the reflection.
- **Katas:** the reflection stays hidden until the problem is solved in that kata.

**UI:**
- tags and a note in the Submissions detail;
- Progress → Mistakes (counts per tag, latest examples, a filter);
- a reflection card after an Accepted Submit;
- the saved reflection at the top of the Notes tab, after a review-mode pass and in the
  kata bar after a solve. This is how "notes per problem" and reflections meet: one Notes
  tab.

**Export:** v2 sections `mistakes` and `reflections`; the newer `updated_at` wins.

## Consequences
- The journal only reads what the judge already stored. No new data leaves the machine.
- A failure's tags are a guess; the user can always change them.
