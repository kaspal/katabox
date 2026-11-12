<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function trimHedge(?TreeNode $root): ?TreeNode {
    return kbTrim($root)[0];
}

/** [the trimmed subtree, its total]. */
function kbTrim(?TreeNode $node): array {
    if ($node === null) {
        return [null, 0];
    }
    [$left, $a] = kbTrim($node->left);
    [$right, $b] = kbTrim($node->right);
    $total = $node->val + $a + $b;
    if ($total < 0) {
        return [null, 0];
    }
    $node->left = $left;
    $node->right = $right;
    return [$node, $total];
}
