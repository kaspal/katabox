# 4. Two SQLite databases

Date: 2026-11-08 · Status: accepted

## Decision
`content.db` (read-only, replaced by pack installs) and `user.db` (progress, WAL mode, numbered migrations, backup before migrating).

## Consequences
Content updates can never damage user progress; export/import only touches `user.db`.
