<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function kthSmallest(?TreeNode $root, int $k): int {
    $stack = [];
    $node = $root;
    while (true) {
        while ($node !== null) {
            $stack[] = $node;
            $node = $node->left;
        }
        $top = array_pop($stack);
        if (--$k === 0) {
            return $top->val;
        }
        $node = $top->right;
    }
}
