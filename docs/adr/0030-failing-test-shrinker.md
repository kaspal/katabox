# 30. Failing-test shrinker

Date: 2026-11-14 · Status: proposed (features-content M3)

## Context
A failing hidden test can be large: hundreds of numbers, a long string, a deep tree.
The bug usually shows on a few values, but finding them by hand is slow. The debugger
(ADR 0022) is much more useful on a small input than a large one.

## Decision
- **"Find a smaller failing input"** appears on a failing test's details after Wrong
  Answer, Runtime Error, Time Limit or Memory Limit. It works only where the input is
  already shown:
  - public tests;
  - custom tests;
  - the reported first failure (ADR 0005's one visible hidden input).

  What a user may see is unchanged.
- **Function problems only.** A program problem says why not: its input carries counts
  and structure lines that can't be cut safely.
- **Candidates** (`kb_judge::shrink`, pure):
  - one parameter changes at a time, and the moves follow the signature's types:
    - arrays: emptied, halves, quarters, single items; each element shrunk when short;
    - numbers: toward 0, ±1, halving;
    - strings: halves and single characters;
    - `ListNode`: as its value list;
    - `TreeNode`: cut only from the end of its level order, with trailing nulls trimmed;
  - every candidate is strictly smaller by a size measure (items, characters, digits),
    so a shrink always ends; the biggest cuts come first.
- **The loop** (kb-app). Each round is one sandbox run per step:
  1. validate.py keeps the valid candidates (if Python is installed);
  2. the reference in the same language gives the expected outputs (as for custom tests;
     inputs it can't answer are dropped);
  3. the user's code runs on them; the first that still fails becomes the new input.

  It stops at the smallest input the moves reach, at **60 sandbox runs or 30 s**, or on
  **Stop**, which is checked before every sandbox run. It holds the judge slot, so it
  never runs alongside a Run, a Submit or the debugger.
- **The result** has the input, expected and actual, the sizes and the number of runs,
  with "Debug this case" and "Add to my tests" (a custom test, in the workspace's state
  like other custom tests).

## Consequences
- **No kb-runner or harness change:** it uses ordinary batch runs of the judge and
  scripts.
- **No new user data:** nothing is stored. The custom test lives in the workspace like
  the others.
- **Cost:** about 3 sandbox runs per round, so a typical shrink is 5-15 rounds.
- **Program problems** may get line or token shrinking later, with a validate.py that
  checks structure.
