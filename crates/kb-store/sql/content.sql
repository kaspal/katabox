-- content.db schema (ADR 0010). Built by `kb-pack build`, read-only at runtime.
-- Changing this file means bumping CONTENT_SCHEMA_VERSION in src/content.rs.

CREATE TABLE content_meta (
    key   TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
) WITHOUT ROWID;

CREATE TABLE problems (
    id            TEXT PRIMARY KEY NOT NULL,
    difficulty    TEXT NOT NULL CHECK (difficulty IN ('easy', 'medium', 'hard')),
    pattern       TEXT NOT NULL,
    rank          INTEGER NOT NULL,
    -- 'function' (the harness calls the solution), 'program' (stdin/stdout; ADR 0028)
    -- or 'sql' (one SELECT over the problem's tables; ADR 0031)
    mode          TEXT NOT NULL DEFAULT 'function' CHECK (mode IN ('function', 'program', 'sql')),
    -- {"name": ..., "params": [{"name": ..., "type": ...}], "returns": ...}; a program
    -- problem stores its synthetic main(stdin: string) -> string
    signature     TEXT NOT NULL,
    checker       TEXT NOT NULL,
    time_limit_ms INTEGER NOT NULL,
    memory_mb     INTEGER NOT NULL,
    -- 1 when the complexity profiler has data for it (profile_inputs; ADR 0024)
    profile       INTEGER NOT NULL DEFAULT 0,
    -- validate.py (constraint checks for custom tests, run in the sandbox; ADR 0016)
    validator     TEXT,
    -- SQL problems only (ADR 0031): schema.sql as written (the judge runs it) and its
    -- tables as JSON for the schema panel and completion
    -- ([{"name", "columns": [{"name", "type", "primary_key", "not_null", "references"}]}])
    schema_sql    TEXT,
    sql_schema    TEXT,
    CHECK ((mode = 'sql') = (schema_sql IS NOT NULL AND sql_schema IS NOT NULL))
) WITHOUT ROWID;

CREATE TABLE problem_tags (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    tag        TEXT NOT NULL,
    PRIMARY KEY (problem_id, tag)
) WITHOUT ROWID;
CREATE INDEX problem_tags_by_tag ON problem_tags (tag);

-- One row per problem per UI language ('en' always present).
CREATE TABLE problem_texts (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    lang       TEXT NOT NULL,
    title      TEXT NOT NULL,
    statement  TEXT NOT NULL,
    editorial  TEXT,
    PRIMARY KEY (problem_id, lang)
) WITHOUT ROWID;

CREATE TABLE hints (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    lang       TEXT NOT NULL,
    tier       INTEGER NOT NULL,
    body       TEXT NOT NULL,
    PRIMARY KEY (problem_id, lang, tier)
) WITHOUT ROWID;

CREATE TABLE tests (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    kind       TEXT NOT NULL CHECK (kind IN ('public', 'hidden')),
    idx        INTEGER NOT NULL,
    -- JSON compressed with zstd, one blob each (ADR 0018; kb_store::blob)
    input      BLOB NOT NULL,
    output     BLOB NOT NULL,
    PRIMARY KEY (problem_id, kind, idx)
) WITHOUT ROWID;

CREATE TABLE reference_solutions (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    language   TEXT NOT NULL,
    code       TEXT NOT NULL,
    PRIMARY KEY (problem_id, language)
) WITHOUT ROWID;

-- The complexity profiler's inputs, one per size n (ADR 0024); JSON compressed with zstd.
CREATE TABLE profile_inputs (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    n          INTEGER NOT NULL,
    input      BLOB NOT NULL,
    PRIMARY KEY (problem_id, n)
) WITHOUT ROWID;

-- The reference solution's median time per call at each size, in microseconds on the
-- reference machine (machine factor 1.0; ADR 0017, 0024).
CREATE TABLE profile_reference (
    problem_id TEXT NOT NULL REFERENCES problems (id),
    language   TEXT NOT NULL,
    n          INTEGER NOT NULL,
    time_us    INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language, n)
) WITHOUT ROWID;

-- Full-text search over both languages; one row per problem per language.
CREATE VIRTUAL TABLE problem_fts USING fts5 (
    problem_id UNINDEXED,
    lang UNINDEXED,
    title,
    tags,
    statement,
    tokenize = 'unicode61 remove_diacritics 2'
);

-- Learn module (ADR 0029): algorithm topics with theory, samples and a quiz. The
-- tracers that animate them are TypeScript in the app, not content.
CREATE TABLE learn_subjects (
    id       TEXT PRIMARY KEY,
    ord      INTEGER NOT NULL,
    name_en  TEXT NOT NULL,
    name_es  TEXT NOT NULL,
    blurb_en TEXT NOT NULL,
    blurb_es TEXT NOT NULL,
    icon     TEXT NOT NULL
) WITHOUT ROWID;

CREATE TABLE learn_topics (
    id            TEXT PRIMARY KEY,
    subject       TEXT NOT NULL REFERENCES learn_subjects (id),
    ord           INTEGER NOT NULL,
    title_en      TEXT NOT NULL,
    title_es      TEXT NOT NULL,
    tags          TEXT NOT NULL,  -- JSON array of strings
    complexity    TEXT NOT NULL,  -- JSON {best, average, worst, space, stable, in_place}
    problems      TEXT NOT NULL,  -- JSON array of problem ids
    presets       TEXT NOT NULL,  -- JSON array of preset ids
    input         TEXT NOT NULL,  -- JSON {min_len, max_len, min, max}
    default_input TEXT NOT NULL,  -- JSON array of integers
    UNIQUE (subject, ord)
) WITHOUT ROWID;

CREATE TABLE learn_texts (
    topic_id TEXT NOT NULL REFERENCES learn_topics (id),
    lang     TEXT NOT NULL,
    theory   TEXT NOT NULL,
    PRIMARY KEY (topic_id, lang)
) WITHOUT ROWID;

-- Raw samples, `@step` marker comments included (the UI strips them).
CREATE TABLE learn_samples (
    topic_id TEXT NOT NULL REFERENCES learn_topics (id),
    language TEXT NOT NULL,
    source   TEXT NOT NULL,
    PRIMARY KEY (topic_id, language)
) WITHOUT ROWID;

-- quiz.json as written (both languages), localized at read time.
CREATE TABLE learn_quiz (
    topic_id TEXT PRIMARY KEY REFERENCES learn_topics (id),
    json     TEXT NOT NULL
) WITHOUT ROWID;

-- Learning paths (schema 7): ordered topic lists shown on Learn home.
CREATE TABLE learn_paths (
    id       TEXT PRIMARY KEY,
    ord      INTEGER NOT NULL,
    title_en TEXT NOT NULL,
    title_es TEXT NOT NULL,
    blurb_en TEXT NOT NULL,
    blurb_es TEXT NOT NULL,
    topics   TEXT NOT NULL  -- JSON array of topic ids, in study order
) WITHOUT ROWID;

CREATE VIRTUAL TABLE learn_fts USING fts5 (
    topic_id UNINDEXED,
    lang UNINDEXED,
    title,
    tags,
    theory,
    tokenize = 'unicode61 remove_diacritics 2'
);
