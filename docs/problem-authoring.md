# Writing a problem

A problem is a folder in `content/problems/<id>/` (CC BY-SA 4.0). Start by copying
`content/problems/two-sum/`. Problems must be original: never copy statements or tests
from LeetCode, HackerRank or similar sites.

## Files

| File | What it holds |
| --- | --- |
| `problem.toml` | Id (equal to the folder name), title, optional `title_es` (Spanish title; English is used without it), difficulty, tags, pattern, rank, the typed `[signature]` and the `[judge]` settings |
| `statement.md`, `statement.es.md` | The statement in English and Spanish |
| `hints.md` | Hints, tiers separated by lines containing only `---` |
| `editorial.md` | The explanation of the solution |
| `hints.es.md`, `editorial.es.md` | Optional Spanish hints and editorial; English is shown without them |
| `tests/public/*.json`, `tests/hidden/*.json` | `{"input": {<param>: value, ...}, "output": value}`, one test per file |
| `solutions/ref.{py,ts,php,rs}` | Reference solutions, standard library only |

## Signature types

`[signature]` declares the function every language implements. Types: `int`, `float`,
`string`, `bool`, and any of these followed by `[]` (nestable, e.g. `int[][]`).

| problem.toml | Python | TypeScript | PHP | Rust |
| --- | --- | --- | --- | --- |
| `int` | `int` | `number` | `int` | `i64` |
| `float` | `float` | `number` | `float` | `f64` |
| `string` | `str` | `string` | `string` | `String` |
| `bool` | `bool` | `boolean` | `bool` | `bool` |
| `T[]` | `list[T]` | `T[]` | `array` | `Vec<T>` |

Names: the function name is camelCase in `problem.toml` (`twoSum`). Python and Rust use
snake_case (`two_sum`); TypeScript and PHP keep camelCase. A name that is a keyword gets a
trailing `_`.

Each reference solution must declare the function exactly as the generated starter code
does (whitespace aside); `kb-pack validate` checks this. To see the starter code:

```sh
cargo run -p kb-judge -- stub <id> --lang <python|typescript|php|rust>
```

## What solutions may use

Standard library only, in every language.

- **PHP** runs as `php -n` (no `php.ini`) with `mbstring` loaded explicitly, so these
  extensions are available: bcmath, calendar, Core, ctype, date, dom, filter, hash, iconv,
  json, lexbor, libxml, mbstring, mysqlnd, pcre, PDO, Phar, random, readline, Reflection,
  session, SimpleXML, SPL, standard, tokenizer, uri, xml, xmlreader, xmlwriter, Zend
  OPcache, zlib. Solutions should stick to the language core (standard, SPL, ctype, json,
  pcre, mbstring, bcmath, random). See ADR 0009.
- **TypeScript** runs on Node's type stripping: no `enum`, `namespace` or constructor
  parameter properties (ADR 0008).
- **Rust** is compiled with `rustc -O --edition 2024` as a single file, so no crates.

## Checking

```sh
cargo run -p kb-pack -- validate        # structure, metadata, tests, signatures
cargo run -p kb-pack -- validate --run  # also judge every reference solution (Windows)
```

To compile every problem into the app's `content.db` (it validates first and refuses
to build if any problem fails):

```sh
cargo run -p kb-pack -- build --out target/content.db
```

`--run` needs the toolchains from `pwsh scripts/fetch-toolchains.ps1` and fails unless
every reference solution is Accepted on every test.
