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

## Decision: tool comforts (M4)
**Snippets** (user.db migration 0008):
- **Data:** `snippets`, keyed by a random 16-hex id so the same snippet merges across
  machines.
- **Fields:** a judge language, a name (at most 80 characters), tags (at most 10; letters,
  digits, `-` and `_`) and a body (at most 64 KB).
- **Inserting:**
  - with `/snip` in the editor: a CodeMirror completion source, added beside the
    language server's;
  - from the command palette ("Insert snippet: …", for the open editor's language),
    through a small editor bridge the workspace registers while it is open.
- **Export:** a v2 section; the newer `updated_at` wins.

**Submission diff:**
- The History tab's merge view moves to `workspace/problem/DiffView.tsx`.
- The Submissions tab gets **Compare**: pick two, and they are shown side by side, the
  older on the left.

**Search everything** (Ctrl+Shift+P, or from the palette):
- **In kb-app** (`search_all`): problems (the existing FTS), Learn topics (FTS) and
  snippets (a LIKE with escaped wildcards), at most 10 of each.
- **In the UI:** the settings (the search index moves to `lib/settingsIndex.ts`) and the
  user guide.
- **Help:** the guide (EN/ES) is bundled with Vite `?raw`, loaded on demand, and shown on
  a new **Help** route by section (`/help?section=<slug>`).
- Nothing is fetched.

## Consequences
- The journal only reads what the judge already stored. No new data leaves the machine.
- A failure's tags are a guess; the user can always change them.
