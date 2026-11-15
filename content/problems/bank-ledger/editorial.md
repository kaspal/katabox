## Approach: one undo log with marks, and a balance → count map

Keep the current state in two hash maps:

- `balance`: name → amount, for `GET`;
- `holders`: amount → how many accounts hold it, for `COUNT` in O(1).

Every change goes through one helper, `put(name, value)`: it takes the account out of `holders` under its old amount (if it existed), then either deletes the account (`value` is "missing") or stores `value` and adds one to `holders[value]`. Keeping both maps behind one door is what keeps `COUNT` right after undos too.

**Transactions.** Instead of copying the ledger at each `BEGIN`, record only what changes:

- `log`: a list of `(name, old balance or missing)`. While any transaction is open, `SET` and `ADD` append the account's old value before calling `put`.
- `marks`: a stack. `BEGIN` pushes the current length of `log`.
- `ROLLBACK` pops a mark `m`, then pops log entries while the log is longer than `m`, calling `put(name, old)` for each.
- `COMMIT` pops the mark and leaves the log alone. Its entries now sit after the enclosing transaction's mark, so they belong to it. When no mark is left, nothing can be rolled back any more, and the log can be cleared.
- With `marks` empty, `ROLLBACK` and `COMMIT` print `NO TRANSACTION`.

**Worked example (Example 2).** `BEGIN` → `marks = [0]`. `SET cy 5` logs `(cy, missing)`. `BEGIN` → `marks = [0, 1]`. `SET cy 9` logs `(cy, 5)`. `COMMIT` pops 1; the log still has both entries. `ROLLBACK` pops 0 and undoes `(cy, 5)`, then `(cy, missing)`: `cy` is gone, so `GET cy` prints `NONE`.

**Why it is correct.** The entries after a mark `m` are exactly the changes made since that `BEGIN`, those of committed inner transactions included (a rolled-back inner transaction has already removed its own). Each entry stores the value just before its change, so undoing them newest first walks the state back one change at a time, to exactly the state at `BEGIN`. An account changed several times has several entries, and the oldest one restores it last. `holders` stays right because every undo goes through `put` as well.

**Complexity:** O(1) expected per command for the maps, plus the undos. Each change adds at most one log entry, and each entry is undone at most once, so all the `ROLLBACK`s together cost O(Q). In all, O(Q) time and memory.

**The obvious approach is too slow.** Copying the whole ledger at every `BEGIN` and scanning every account for each `COUNT` costs O(accounts) per command: with 2.5·10⁴ accounts and 8·10⁴ commands, many of them `BEGIN` or `COUNT`, that is about 10⁹ steps.

**Practical notes:** read the whole input at once, split it into lines, and print all answers in one write. `COUNT` may ask for an amount nobody holds, and an account holding 0 still counts for `COUNT 0` (a missing one does not). Balances reach 10⁵ · 10⁹ = 10¹⁴ in absolute value and may be negative: use `i64` in Rust; a TypeScript `Number` is exact up to 2⁵³, and a `Map<number, number>` works for `holders`; PHP integers are 64-bit. Read `amount` with a sign (`-20`).
