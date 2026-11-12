// ---- Node types (ADR 0025): JSON <-> ListNode / TreeNode -------------------------------
// A list travels as its values in order; a tree as its level order with null for a missing
// child (children of a missing node are not listed, trailing nulls trimmed). Nodes are
// built with the solution's own exported classes, without running their constructors, and
// read back by duck typing; everything is iterative, so deep structures are fine.

const NODE_LIMIT = 1_000_000;

/** A returned structure the judge refuses (a cycle, or too many nodes). */
class NodeError extends Error {
  readonly code: string;
  constructor(code: string, message: string) {
    super(message);
    this.code = code;
  }
}

type AnyNode = Record<string, unknown>;
type Classes = { list: { prototype: object }; tree: { prototype: object } };

class KbListNode {
  val: number;
  next: KbListNode | null;
  constructor(val = 0, next: KbListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

class KbTreeNode {
  val: number;
  left: KbTreeNode | null;
  right: KbTreeNode | null;
  constructor(val = 0, left: KbTreeNode | null = null, right: KbTreeNode | null = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

/** The solution's exported ListNode and TreeNode classes, or the judge's own. */
function nodeClasses(mod: Record<string, unknown>): Classes {
  const pick = (name: string, fallback: { prototype: object }) =>
    typeof mod[name] === "function" ? (mod[name] as { prototype: object }) : fallback;
  return { list: pick("ListNode", KbListNode), tree: pick("TreeNode", KbTreeNode) };
}

function makeNode(cls: { prototype: object }, fields: AnyNode): AnyNode {
  return Object.assign(Object.create(cls.prototype), fields);
}

function listFromJson(cls: { prototype: object }, values: number[]): AnyNode | null {
  let head: AnyNode | null = null;
  for (let i = values.length - 1; i >= 0; i--) head = makeNode(cls, { val: values[i], next: head });
  return head;
}

function treeFromJson(cls: { prototype: object }, values: (number | null)[]): AnyNode | null {
  if (values.length === 0) return null;
  const nodes = values.map((v) => (v === null ? null : makeNode(cls, { val: v, left: null, right: null })));
  let i = 1;
  for (const parent of nodes) {
    if (i >= nodes.length) break;
    if (parent === null) continue;
    parent.left = nodes[i++];
    if (i < nodes.length) parent.right = nodes[i++];
  }
  return nodes[0];
}

function listToJson(node: unknown): unknown[] {
  const out: unknown[] = [];
  const seen = new Set<unknown>();
  let cur = node as AnyNode | null | undefined;
  while (cur !== null && cur !== undefined) {
    if (seen.has(cur)) throw new NodeError("node_cycle", "the returned list links back to a node it already contains (a cycle)");
    seen.add(cur);
    if (out.length >= NODE_LIMIT) throw new NodeError("return_not_serializable", `the returned list has more than ${NODE_LIMIT} nodes`);
    if (typeof cur !== "object" || !("val" in cur)) throw new TypeError("not a ListNode: " + String(cur));
    out.push(cur.val);
    cur = cur.next as AnyNode | null | undefined;
  }
  return out;
}

function treeToJson(root: unknown): unknown[] {
  if (root === null || root === undefined) return [];
  const out: unknown[] = [];
  const seen = new Set<unknown>();
  const queue: unknown[] = [root];
  for (let i = 0; i < queue.length; i++) {
    const node = queue[i] as AnyNode | null | undefined;
    if (node === null || node === undefined) {
      out.push(null);
      continue;
    }
    if (seen.has(node)) throw new NodeError("node_cycle", "the returned tree reaches the same node twice (a cycle or a shared node)");
    seen.add(node);
    if (seen.size > NODE_LIMIT) throw new NodeError("return_not_serializable", `the returned tree has more than ${NODE_LIMIT} nodes`);
    if (typeof node !== "object" || !("val" in node)) throw new TypeError("not a TreeNode: " + String(node));
    out.push(node.val);
    queue.push(node.left ?? null, node.right ?? null);
  }
  while (out.length > 0 && out[out.length - 1] === null) out.pop();
  return out;
}

/** A JSON value as the solution receives it (`shape` is "list", "tree" or "plain"). */
function fromJson(shape: string, depth: number, value: unknown, classes: Classes): unknown {
  if (shape === "plain") return value;
  if (depth > 0) return (value as unknown[]).map((v) => fromJson(shape, depth - 1, v, classes));
  return shape === "list" ? listFromJson(classes.list, value as number[]) : treeFromJson(classes.tree, value as (number | null)[]);
}

/** A returned value as JSON (throws NodeError, or TypeError for a non-node). */
function toJson(shape: string, depth: number, value: unknown): unknown {
  if (shape === "plain") return value;
  if (depth > 0) {
    if (!Array.isArray(value)) throw new TypeError("expected an array, got " + typeof value);
    return value.map((v) => toJson(shape, depth - 1, v));
  }
  return shape === "list" ? listToJson(value) : treeToJson(value);
}
