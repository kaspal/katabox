<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function longestWalk(?TreeNode $root): int {
    $best = 0;
    $depth = function (?TreeNode $node) use (&$depth, &$best): int {
        if ($node === null) {
            return 0;
        }
        $a = $depth($node->left);
        $b = $depth($node->right);
        $best = max($best, $a + $b);
        return 1 + max($a, $b);
    };
    $depth($root);
    return $best;
}
