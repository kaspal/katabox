<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function richestTrail(?TreeNode $root): int {
    $best = PHP_INT_MIN;
    $gain = function (?TreeNode $node) use (&$gain, &$best): int {
        if ($node === null) {
            return 0;
        }
        $a = max(0, $gain($node->left));
        $b = max(0, $gain($node->right));
        $best = max($best, $node->val + $a + $b);
        return $node->val + max($a, $b);
    };
    $gain($root);
    return $best;
}
