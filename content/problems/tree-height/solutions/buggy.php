<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function treeHeight(?TreeNode $root): int {
    if ($root === null) {
        return 1;
    }
    return 1 + max(treeHeight($root->left), treeHeight($root->right));
}
