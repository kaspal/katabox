<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function fullestLevel(?TreeNode $root): int {
    if ($root === null) {
        return 0;
    }
    $best = -1;
    $bestLevel = 0;
    $level = 0;
    $queue = [$root];
    while ($queue !== []) {
        $level++;
        $total = 0;
        $next = [];
        foreach ($queue as $node) {
            $total += $node->val;
            if ($node->left !== null) {
                $next[] = $node->left;
            }
            if ($node->right !== null) {
                $next[] = $node->right;
            }
        }
        if ($total > $best) {
            $best = $total;
            $bestLevel = $level;
        }
        $queue = $next;
    }
    return $bestLevel;
}
