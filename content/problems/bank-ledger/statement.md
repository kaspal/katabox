A small bank keeps its accounts in an electronic ledger: each account has a name and a whole-number balance. Clerks type commands to change balances and to look them up. Changes can be grouped in transactions, which can be nested: a clerk may open a transaction inside another, and later either cancel or keep the changes of the innermost one.

This is a **program problem**: read the commands from standard input and print the answers to standard output.

## Input format

- The first line holds `Q`, the number of commands.
- Then `Q` lines, each one of these commands:
  - `SET name amount`: the account `name` now holds `amount`, creating it if it does not exist.
  - `ADD name amount`: add `amount` (which may be negative) to the balance of `name`. If the account does not exist, it is created holding `amount`.
  - `GET name`: print the balance of `name`, or `NONE` if the account does not exist.
  - `COUNT amount`: print how many existing accounts hold exactly `amount`.
  - `BEGIN`: open a new transaction inside the ones already open.
  - `ROLLBACK`: undo every change made since the innermost open transaction began (including changes of transactions inside it that were committed), and close it. An account created in that time disappears again.
  - `COMMIT`: close the innermost open transaction and keep its changes. They now belong to the enclosing transaction, if any, so a later `ROLLBACK` of that one still undoes them.

  `ROLLBACK` and `COMMIT` with no transaction open print `NO TRANSACTION` and change nothing. Transactions still open at the end are simply left open.

## Output format

Print one line for every `GET`, every `COUNT`, and every `ROLLBACK` or `COMMIT` with no transaction open, in the order of the commands. The other commands print nothing.

### Example 1

Input:

```
10
SET ana 50
SET bo 50
COUNT 50
BEGIN
ADD ana -20
GET ana
COUNT 50
ROLLBACK
GET ana
COUNT 50
```

Output:

```
2
30
1
50
2
```

Inside the transaction `ana` drops to 30, so only `bo` holds 50. The `ROLLBACK` puts `ana` back to 50.

### Example 2

Input:

```
7
BEGIN
SET cy 5
BEGIN
SET cy 9
COMMIT
ROLLBACK
GET cy
```

Output:

```
NONE
```

The `COMMIT` hands the change to 9 to the outer transaction, and its `ROLLBACK` undoes both changes: `cy` did not exist before it began.

### Example 3

Input:

```
6
COMMIT
ROLLBACK
ADD dee 7
GET dee
COUNT 0
GET zed
```

Output:

```
NO TRANSACTION
NO TRANSACTION
7
0
NONE
```

`ADD` creates `dee` with 7. Accounts that do not exist are not counted by `COUNT 0`.

### Constraints

- 1 ≤ Q ≤ 10⁵
- Each `name` has 1 to 10 lowercase letters `a` to `z`.
- `SET`: 0 ≤ amount ≤ 10⁹
- `ADD`: -10⁹ ≤ amount ≤ 10⁹
- `COUNT`: -10¹⁴ ≤ amount ≤ 10¹⁴
