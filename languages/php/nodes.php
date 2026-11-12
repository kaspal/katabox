// ---- Node types (ADR 0025): JSON <-> ListNode / TreeNode -------------------------------
// A list travels as its values in order; a tree as its level order with null for a missing
// child (children of a missing node are not listed, trailing nulls trimmed). Nodes are
// built with the solution's own classes (its starter code defines them; the judge declares
// them only if it does not), without running their constructors, and read back by their
// public properties; everything is iterative, so deep structures are fine.

const KB_NODE_LIMIT = 1000000;

/** A returned structure the judge refuses (a cycle, or too many nodes). */
final class KbNodeError extends Exception
{
    public function __construct(public string $kbCode, string $message)
    {
        parent::__construct($message);
    }
}

function kb_declare_nodes(): void
{
    if (!class_exists('ListNode', false)) {
        class ListNode
        {
            public function __construct(public int $val = 0, public ?ListNode $next = null)
            {
            }
        }
    }
    if (!class_exists('TreeNode', false)) {
        class TreeNode
        {
            public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
            {
            }
        }
    }
}

function kb_node(string $class, array $fields): object
{
    static $reflection = [];
    $reflection[$class] ??= new ReflectionClass($class);
    $node = $reflection[$class]->newInstanceWithoutConstructor();
    foreach ($fields as $key => $value) {
        $node->$key = $value;
    }
    return $node;
}

function kb_list_from_json(array $values): ?object
{
    $head = null;
    for ($i = count($values) - 1; $i >= 0; $i--) {
        $head = kb_node('ListNode', ['val' => $values[$i], 'next' => $head]);
    }
    return $head;
}

function kb_tree_from_json(array $values): ?object
{
    if ($values === []) {
        return null;
    }
    $nodes = array_map(
        fn ($v) => $v === null ? null : kb_node('TreeNode', ['val' => $v, 'left' => null, 'right' => null]),
        $values
    );
    $n = count($nodes);
    $i = 1;
    foreach ($nodes as $parent) {
        if ($i >= $n) {
            break;
        }
        if ($parent === null) {
            continue;
        }
        $parent->left = $nodes[$i++];
        if ($i < $n) {
            $parent->right = $nodes[$i++];
        }
    }
    return $nodes[0];
}

function kb_list_to_json(mixed $node): array
{
    $out = [];
    $seen = [];
    while ($node !== null) {
        if (!is_object($node)) {
            throw new TypeError('not a ListNode: ' . get_debug_type($node));
        }
        $id = spl_object_id($node);
        if (isset($seen[$id])) {
            throw new KbNodeError('node_cycle', 'the returned list links back to a node it already contains (a cycle)');
        }
        $seen[$id] = true;
        if (count($out) >= KB_NODE_LIMIT) {
            throw new KbNodeError('return_not_serializable', 'the returned list has more than ' . KB_NODE_LIMIT . ' nodes');
        }
        $out[] = $node->val;
        $node = $node->next;
    }
    return $out;
}

function kb_tree_to_json(mixed $root): array
{
    if ($root === null) {
        return [];
    }
    $out = [];
    $seen = [];
    $queue = [$root];
    for ($i = 0; $i < count($queue); $i++) {
        $node = $queue[$i];
        if ($node === null) {
            $out[] = null;
            continue;
        }
        if (!is_object($node)) {
            throw new TypeError('not a TreeNode: ' . get_debug_type($node));
        }
        $id = spl_object_id($node);
        if (isset($seen[$id])) {
            throw new KbNodeError('node_cycle', 'the returned tree reaches the same node twice (a cycle or a shared node)');
        }
        $seen[$id] = true;
        if (count($seen) > KB_NODE_LIMIT) {
            throw new KbNodeError('return_not_serializable', 'the returned tree has more than ' . KB_NODE_LIMIT . ' nodes');
        }
        $out[] = $node->val;
        $queue[] = $node->left;
        $queue[] = $node->right;
    }
    while ($out !== [] && end($out) === null) {
        array_pop($out);
    }
    return $out;
}

/** A JSON value as the solution receives it ($shape is "list", "tree" or "plain"). */
function kb_from_json(string $shape, int $depth, mixed $value): mixed
{
    if ($shape === 'plain') {
        return $value;
    }
    if ($depth > 0) {
        return array_map(fn ($v) => kb_from_json($shape, $depth - 1, $v), $value);
    }
    return $shape === 'list' ? kb_list_from_json($value) : kb_tree_from_json($value);
}

/** A returned value as JSON (throws KbNodeError, or TypeError for a non-node). */
function kb_to_json(string $shape, int $depth, mixed $value): mixed
{
    if ($shape === 'plain') {
        return $value;
    }
    if ($depth > 0) {
        if (!is_array($value)) {
            throw new TypeError('expected an array, got ' . get_debug_type($value));
        }
        return array_map(fn ($v) => kb_to_json($shape, $depth - 1, $v), array_values($value));
    }
    return $shape === 'list' ? kb_list_to_json($value) : kb_tree_to_json($value);
}
