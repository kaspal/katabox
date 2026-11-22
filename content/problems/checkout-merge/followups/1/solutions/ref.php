<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function mergeDistinct(array $lines): ?ListNode {
    $tickets = [];
    foreach ($lines as $node) {
        while ($node !== null) {
            $tickets[] = $node->val;
            $node = $node->next;
        }
    }
    sort($tickets);
    $dummy = new ListNode();
    $tail = $dummy;
    foreach ($tickets as $i => $t) {
        if ($i > 0 && $t === $tickets[$i - 1]) {
            continue;
        }
        $tail->next = new ListNode($t);
        $tail = $tail->next;
    }
    return $dummy->next;
}
