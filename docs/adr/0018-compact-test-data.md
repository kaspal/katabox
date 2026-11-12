# 18. Compact test data

Date: 2026-11-11 · Status: proposed

## Context
- **Storage today:** problem tests are stored in content.db as raw JSON. The first 10
  problems held 7.1 MB of tests, and content.db was 7.7 MB.
- **Growth:** max-size tests (ADR 0016) are needed to force the intended complexity in
  Python and Rust (ADR 0017), so about 1 MB per problem is common. 150 problems would make
  content.db about 150 MB, shipped with every installer.
- **Measurements** on those 10 problems:

  | Encoding | Total | Largest problem |
  | --- | --- | --- |
  | raw JSON | 7270 KB | rainfall-totals 2043 KB |
  | deflate 9 | 2505 KB | two-sum 489 KB |
  | zstd 19, per test | 2174 KB | two-sum 465 KB |
  | zstd 19, per problem | 2155 KB | |
  | xz 9 | 2110 KB | |

- **Why compression only goes so far:** max tests are mostly random numbers, and those
  barely compress. Compression gives about 3.3×. Beyond that, the lever is how authors
  write max tests.

## Decision

### zstd per test (content.db schema v3)
- **Format:** `tests.input` and `tests.output` are `BLOB`s holding compact JSON compressed
  with zstd at level 19 (`kb_store::blob`). `CONTENT_SCHEMA_VERSION` is 3.
- **No user migration:** the app ships its own content.db, so nothing migrates.
- **Per test, not per problem:** the ratio is the same (2174 vs 2155 KB). The problem page
  reads the public tests without decompressing the hidden ones, and the row layout is
  unchanged.
- **Reading:** decompression is capped at 64 MiB per value, so a corrupt or hostile pack
  cannot exhaust memory. A bad blob is a `Corrupt` store error.
- **Library:** the `zstd` crate (MIT; zstd-sys MIT/Apache; libzstd BSD), all on the license
  allow-list.
  - It compiles C, as the bundled SQLite already does.
  - Debug builds compile it optimized (`[profile.dev.package.zstd-sys]`), so building
    content.db stays fast in development.
  - deflate (pure Rust, already a dependency) was 15% larger.
- **Result:** content.db for the first 10 problems went from 7.72 MB to 2.49 MB. With all
  25 problems (15.3 MB of test JSON) it is 5.15 MB.

### A size budget per problem
- **The check:** `kb-pack validate` (static, so CI enforces it) prints every problem's test
  size, as JSON and compressed with the same function the build uses. A problem over
  **512 KB compressed** fails.
- **Current state:** today's largest is two-sum at 465 KB (90%). The current problems were
  kept as they are.
- **Projection:** at the 25-problem average (about 180 KB compressed), 150 problems would
  make content.db about 30 MB.
- **Guidance for authors** (`docs/problem-authoring.md`):
  - use the smallest value range that keeps the problem honest;
  - remember that the CPU cap covers the whole run, so one max test often suffices;
  - outputs count too;
  - structured data (sorted, repeated, arithmetic) compresses far better than random
    values.

### Max tests stay committed as JSON
Generating max tests at build time with `kb-pack generate` (fixed seeds, verified by
hash) was considered and rejected.

| | Committed JSON (chosen) | Generated at build time |
| --- | --- | --- |
| Build needs | nothing beyond Rust | the Python toolchain and the sandbox, in CI and for every desktop build (`build.rs` builds content.db), on Windows and Linux |
| Reproducibility | exact | depends on `random.Random` and float formatting staying identical across Python versions; a hash mismatch would block releases |
| Review | the data is in the diff | reviewers see seeds; the data appears only after running scripts |
| Repository size | grows by the raw JSON, but git stores objects zlib-compressed (about 2.5 MB for the first 10 problems), and the budget bounds it | smaller |

The repository cost is bounded and modest. Generating at build time would add a heavy,
fragile build dependency. Committed JSON is clearly the better option.

## Consequences
- The app's installer and content packs shrink about 3×, and new problems have a hard,
  visible size limit.
- Test data in content.db is no longer readable with a plain SQLite browser; use
  `kb_store::blob` or the app.
- Changing the zstd level or library changes content.db bytes but not the schema; changing
  the storage format means bumping `CONTENT_SCHEMA_VERSION`.

## Follow-ups
- **Provenance:** `kb-pack validate --run` could regenerate each generated test from a
  recorded seed and compare it with the committed file.
- **A zstd dictionary** trained on test data, if many small tests ever dominate.
