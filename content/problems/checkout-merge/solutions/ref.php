<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function mergeLines(array $lines): ?ListNode {
    $fronts = array_values($lines);
    // SplMinHeap compares [ticket, line] arrays element by element.
    $heap = new SplMinHeap();
    foreach ($fronts as $i => $node) {
        if ($node !== null) {
            $heap->insert([$node->val, $i]);
        }
    }
    $dummy = new ListNode();
    $tail = $dummy;
    while (!$heap->isEmpty()) {
        [, $i] = $heap->extract();
        $node = $fronts[$i];
        $fronts[$i] = $node->next;
        $tail->next = $node;
        $tail = $node;
        if ($fronts[$i] !== null) {
            $heap->insert([$fronts[$i]->val, $i]);
        }
    }
    $tail->next = null;
    return $dummy->next;
}
