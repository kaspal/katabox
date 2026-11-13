<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function isSearchTree(?TreeNode $root): bool {
    $ok = function (?TreeNode $node, ?int $low, ?int $high) use (&$ok): bool {
        if ($node === null) {
            return true;
        }
        if (($low !== null && $node->val <= $low) || ($high !== null && $node->val >= $high)) {
            return false;
        }
        return $ok($node->left, $low, $node->val) && $ok($node->right, $node->val, $high);
    };
    return $ok($root, null, null);
}
