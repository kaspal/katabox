# 26. A faster Rust harness reader

Date: 2026-11-13 · Status: accepted

## Context
- **The Rust harness spent more CPU reading the request than the solutions spent
  solving it.** With the 150-problem catalogue, many Rust requests are 1-2 MB of JSON (all
  of a problem's tests in one request, ADR 0008).
- **Measured** with the real rendered harness, `rustc -O` as in `language.toml`, outside
  the sandbox, as the median of 5 runs:

  | Phase (undo-notebook, 1.96 MB) | ms |
  | --- | --- |
  | parse into a `Json` tree | 32 |
  | `tests.clone()` | 30 |
  | `from_json` into the argument types | 21 |
  | freeing the cloned tree | 13 |
  | the solution itself | 15 |
  | process CPU in all | 141 |

  The other large problems looked the same: the reader was about two thirds of the CPU.
- **Not the compiler flags:** `-C opt-level=3` measured the same as `-O`.
- **The cause was allocation and copying:**
  - one heap `String` per number, parsed again by `from_json`;
  - one `Vec` per small array, built twice (tree, then typed);
  - a deep copy of the tree, freed node by node.
- **The effect:** Rust references sat at 109-171 ms of their 200 ms allowance, and
  undo-notebook failed `validate --run` now and then.

## Decision
- **The reader moves to `languages/rust/json.rs`:** template-free Rust, standard library
  only.
  - kb-judge injects it into every Rust harness as `{{ json_code }}`, as it does
    `nodes.rs`.
  - kb-judge's tests `include!` the same file, so the tested code is the shipped code.
- **The fast path reads test inputs straight into the argument types** (`ReadJson`):
  - each test object is read into the signature's `(T0, T1, …)`, with no tree in between;
  - integers are parsed once;
  - unescaped string runs are copied in bulk.
- **The generic path is unchanged and stays the authority.**
  - When a value does not fit its type (wrong type, missing key, a test that is not an
    object), the reader rewinds to the start of that test and runs `parse` + `FromJson`
    as before.
  - So acceptance, the per-test "harness: bad input" messages and value formatting in
    them are exactly the old ones. `Json`'s `Debug` keeps printing integers as
    `Num("7")`.
  - Node types (`ListNode`, `TreeNode`) always take the generic path.
- **No clone and no teardown:** the tests are moved, never copied, and the process exits
  without freeing the request.
- **The request is read on the worker thread.** The worker is the 64 MB-stack thread
  that runs the tests, because `Rc` tree arguments cannot cross threads.
- **Nesting is capped at 10 000 levels.** A deeper request now exits 70 cleanly; before,
  it overflowed the 1 MB main-thread stack and crashed. Below the cap nothing changes:
  real requests nest a few levels.

**The protocol is unchanged (ADR 0008):**
- the same records, nonce and `max_output_bytes`;
- exit 70 for unreadable or malformed requests and a missing nonce;
- the same panic hook and messages;
- the whole request is still read and checked before the first test runs.

## Consequences
- **Rust CPU on the large requests drops 2-3.5×:**

  | Problem | before | after |
  | --- | --- | --- |
  | undo-notebook | 125-141 ms | 47 ms |
  | orchard-plots | 109 ms | 31 ms |
  | library-holds | 109-125 ms | 47 ms |

  Small requests stay at the timer floor. The before/after table for the slowest
  problems and the naive margins are in the hardening report.
- **naive.rs gets faster too,** by the same harness saving. Every problem still needs
  naive.rs ≥ 1.5× its cap (`kb-pack validate --run`).
- **Correctness is checked by `crates/kb-judge/tests/rust_reader.rs`:**
  - every test file of every problem, and whole requests, read the same as serde_json
    reads them;
  - every test input takes the fast path with the generic path's result;
  - wrong-type values get the generic answer;
  - edge cases: escapes, surrogates, non-ASCII, ±(2⁵³ − 1), exponents, empty and nested
    containers, whitespace;
  - malformed input fails without panicking, including every prefix of real files.
- **Old and new harnesses were compared end to end** on 22 problems, and their records
  were identical apart from timings. That covers int, float, string, nested arrays,
  `ListNode`, `ListNode[]` and `TreeNode` in and out, plus malformed requests and their
  exit codes.
- **The harness source changed,** so cached Rust binaries rebuild once (the cache key
  covers it, ADR 0009).
