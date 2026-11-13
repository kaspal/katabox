<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function windowView(?TreeNode $root): array {
    $seen = [];
    $level = $root === null ? [] : [$root];
    while ($level !== []) {
        $seen[] = $level[count($level) - 1]->val;
        $next = [];
        foreach ($level as $node) {
            if ($node->left !== null) {
                $next[] = $node->left;
            }
            if ($node->right !== null) {
                $next[] = $node->right;
            }
        }
        $level = $next;
    }
    return $seen;
}
