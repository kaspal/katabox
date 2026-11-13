<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function mergeQueues(?ListNode $first, ?ListNode $second): ?ListNode {
    $head = new ListNode();
    $tail = $head;
    while ($first !== null && $second !== null) {
        if ($first->val <= $second->val) {
            $tail->next = $first;
            $first = $first->next;
        } else {
            $tail->next = $second;
            $second = $second->next;
        }
        $tail = $tail->next;
    }
    $tail->next = $first ?? $second;
    return $head->next;
}
