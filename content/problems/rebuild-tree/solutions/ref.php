<?php
// Provided by KataBox: the judge builds the input trees with this class.
class TreeNode
{
    public function __construct(public int $val = 0, public ?TreeNode $left = null, public ?TreeNode $right = null)
    {
    }
}

function rebuildTree(array $preorder, array $inorder): ?TreeNode {
    $pos = array_flip($inorder);
    $build = function (int $ps, int $ins, int $length) use (&$build, $preorder, $pos): ?TreeNode {
        if ($length === 0) {
            return null;
        }
        $root = new TreeNode($preorder[$ps]);
        $left = $pos[$root->val] - $ins;
        $root->left = $build($ps + 1, $ins, $left);
        $root->right = $build($ps + 1 + $left, $ins + $left + 1, $length - $left - 1);
        return $root;
    };
    return $build(0, 0, count($preorder));
}
