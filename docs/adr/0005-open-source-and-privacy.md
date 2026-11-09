# 5. Open source, no telemetry, hidden tests are not secret

Date: 2026-11-08 · Status: accepted

## Decision
Code under MIT OR Apache-2.0; problem content under CC BY-SA 4.0. Only permissively licensed tools are bundled (phpactor instead of intelephense). No telemetry or accounts. Hidden tests ship as plain data; after a failed submit the first failing hidden input is revealed. The AI tutor is local-only (Ollama).

## Consequences
Anyone can fork and contribute; there is no anti-cheat, by design. The repository stays private until the v1.0 beta, then becomes public.
