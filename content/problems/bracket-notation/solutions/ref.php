<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function bracketNotation(?TreeNode $root): string {
    $parts = [];
    $write = function (TreeNode $node) use (&$write, &$parts): void {
        $parts[] = (string) $node->val;
        if ($node->left !== null || $node->right !== null) {
            $parts[] = "(";
            if ($node->left !== null) {
                $write($node->left);
            }
            $parts[] = ")";
        }
        if ($node->right !== null) {
            $parts[] = "(";
            $write($node->right);
            $parts[] = ")";
        }
    };
    if ($root !== null) {
        $write($root);
    }
    return implode("", $parts);
}
