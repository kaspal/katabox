<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function reverseBatches(?ListNode $line, int $k): ?ListNode {
    $dummy = new ListNode(0, $line);
    $before = $dummy;
    while (true) {
        $probe = $before;
        for ($i = 0; $i < $k; $i++) {
            $probe = $probe->next;
            if ($probe === null) {
                return $dummy->next;
            }
        }
        $first = $before->next;
        $prev = null;
        $node = $first;
        for ($i = 0; $i < $k; $i++) {
            $next = $node->next;
            $node->next = $prev;
            $prev = $node;
            $node = $next;
        }
        $before->next = $prev;
        $first->next = $node;
        $before = $first;
    }
}
