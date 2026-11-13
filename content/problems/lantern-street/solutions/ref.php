<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function brightestStreet(?TreeNode $root): int {
    $best = function (?TreeNode $node) use (&$best): array {
        if ($node === null) {
            return [0, 0];
        }
        [$lLit, $lDark] = $best($node->left);
        [$rLit, $rDark] = $best($node->right);
        return [$node->val + $lDark + $rDark, max($lLit, $lDark) + max($rLit, $rDark)];
    };
    return max($best($root));
}
