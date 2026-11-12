# 25. Linked-list and binary-tree types (ListNode, TreeNode)

Date: 2026-11-12 · Status: accepted

## Context
- **The plan's type vocabulary** includes `ListNode` and `TreeNode`. The judge supported
  only `int`, `float`, `string`, `bool` and arrays of them.
- **Linked-list problems** could not be written at all.
- **Trees** were stood in for by parent arrays.
- **Harnesses passed raw JSON:**
  - Python and TypeScript handed the solution the JSON values;
  - PHP knew only the return value's array depth;
  - only Rust decoded and encoded by type.

## Decision

### Types and JSON encoding
- **Two new signature types,** `ListNode` (a singly linked list of `int`) and `TreeNode`
  (a binary tree of `int`). Arrays of them work as for any type (`ListNode[]`).
- **A list** travels as its values in order: `[1, 2, 3]` is 1 → 2 → 3, and `[]` is the
  empty list (null).
- **A tree** travels in level order, top to bottom and left to right:
  - `null` stands for a missing child;
  - the children of a missing node are not listed;
  - trailing nulls are trimmed;
  - `[]` is the empty tree.
- **Examples:**
  - `[1, null, 2, 3]` is 1 with a right child 2, whose left child is 3;
  - `[5, 3, 8, 1, null, 7, 9]` is a full first two levels, plus 3's left child 1 and
    8's children 7 and 9.
- **Values are `int`** (i64; the UI keeps them JavaScript-safe).
- **Harnesses always write this canonical form**, so the existing checkers compare the
  JSON as before (`exact`; `unordered` makes no sense for trees).
- **`Type::check_value`** checks any value against its type. A tree must be canonical:
  - the root is not null;
  - every slot is owned by a node listed before it (each node owns two);
  - there is no trailing null.
- **Where values are checked:**
  - `check_input`: problem tests, custom inputs and debug inputs, with the param named in
    the message;
  - kb-pack, on every test input **and** output, for all types. All 70 earlier problems
    pass.

### Per-language mapping

| Type | Python | TypeScript | PHP | Rust |
| --- | --- | --- | --- | --- |
| `ListNode` | `ListNode \| None` | `ListNode \| null` | `?ListNode` | `Option<Box<ListNode>>` |
| `TreeNode` | `TreeNode \| None` | `TreeNode \| null` | `?TreeNode` | `Option<Rc<RefCell<TreeNode>>>` |
| `ListNode[]` | `list[ListNode \| None]` | `(ListNode \| null)[]` | `array` | `Vec<Option<Box<ListNode>>>` |

### The classes live in the starter code (the maintainer's choice)
- **The stub defines what it uses before the function:**
  - Python: a class with `__init__(val=0, next=None)`;
  - TypeScript: an `export class` with explicit fields, since parameter properties are
    not erasable syntax;
  - PHP: a class with promoted constructor properties;
  - Rust: `pub struct ListNode { pub val: i64, pub next: Option<Box<ListNode>> }` and
    `pub struct TreeNode { pub val: i64, pub left/right: Option<Rc<RefCell<TreeNode>>> }`,
    each with `new`.
- **Why there:**
  - the language servers see the definitions, so there are no "undefined name" errors
    and the completions work;
  - users read the exact shape and can construct nodes.
- **Reference solutions** contain the same definitions. kb-pack compares the
  **function's declaration line** (`kb_judge::signature_line`) instead of the stub's
  first line.

### Harnesses
- **Building the inputs:**
  - uses the solution's own `ListNode`/`TreeNode` classes;
  - **without running their constructors** (Python `cls.__new__`, TS `Object.create`,
    PHP `ReflectionClass::newInstanceWithoutConstructor`; Rust uses `new` and sets the
    fields);
  - is iterative, so a 10⁵-node list or a 10⁴-deep tree is fine.
- **The class fallback:** when a solution drops the definition, Python, TypeScript and
  PHP fall back to the judge's own class. Rust needs the structs.
- **Writing the results:** iterative and duck-typed on `val`/`next`/`left`/`right`.
  Nodes the solution created are written like the input ones.
- **Guards (each a Runtime Error, never a hang or a crash):**
  - a node reached twice (a list cycle, or a tree node shared by two parents) gives the
    new message code **`node_cycle`**;
  - more than 10⁶ nodes gives `return_not_serializable`.
- **Rust:** the `FromJson`/`ToJson` impls are emitted only for signatures that use
  nodes. Other problems' harnesses, and their compile-cache keys, are unchanged. A `Box`
  list cannot cycle; an `Rc` tree can, and is checked by pointer.
- **The debug harnesses** (Python, TypeScript) use the same conversion code.

### Debugger display (Python and TypeScript)
- **Which values:** an instance of a class named `ListNode` or `TreeNode` that has the
  node fields.
- **Its value line is short and cycle-safe,** never the whole chain or an address:
  - `ListNode [4 → 8 → 15 → …]`: the first 10 values, then "↺" on a loop;
  - `TreeNode(3, left=-5, right=2)`.
- **Its children,** in a fixed order: `val`, `next` (or `val`, `left`, `right`). Each
  expands lazily through the existing paths.
- **No change** to the kb-core types or the UI panels.

### Language servers
- **Python:** basedpyright's standard mode reports attribute access on `ListNode | None`
  (`head.next.next`) as an error, even in correct solutions. The Python project's
  `pyrightconfig.json` therefore sets `"reportOptionalMemberAccess": "warning"`
  (maintainer-approved; recorded in ADR 0014's notes). Real errors stay errors.
- **TypeScript:** strict mode reports the same pattern. The editor's JS mode (ADR 0024)
  is the lenient option there.

### UI
- **Custom inputs** accept lists and canonical trees (`matchesType`).
- **A type error on a node param** explains the JSON form, with an example
  (`results.custom.errors.list` / `tree`, EN and ES).

## Consequences
- **Linked-list and binary-tree problems can be written.** Four new problems use the
  types:
  - Playlist Rotation and Closing Checkouts (linked lists);
  - Fullest Level and Hedge Trim (trees).
- **Recursion depth:** tree problems keep their height at most 1000, so recursive
  solutions work in every language (Python's limit is 10 000). The deep cases are tested
  in the harness round trips, not as content.
- **Rust drops a long `Box` list recursively.** That is fine in the harness's 64 MB
  thread at 10⁵ nodes. Problems stay at or below that.
- **Typing every test value** catches wrong outputs in content early. It is also a new
  rule for all problems.
