<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function fairSample(array $lines): array {
    $heap = new SplPriorityQueue();
    $heap->setExtractFlags(SplPriorityQueue::EXTR_DATA);
    $largest = 0;
    foreach ($lines as $i => $node) {
        // SplPriorityQueue is a max-heap: negate to pop the smallest ticket first.
        $heap->insert([$node, $i], [-$node->val, -$i]);
        $largest = max($largest, $node->val);
    }
    $best = null;
    while (true) {
        [$node, $i] = $heap->extract();
        $low = $node->val;
        if ($best === null || $largest - $low < $best[1] - $best[0]) {
            $best = [$low, $largest];
        }
        if ($node->next === null) {
            return $best;
        }
        $node = $node->next;
        $largest = max($largest, $node->val);
        $heap->insert([$node, $i], [-$node->val, -$i]);
    }
}
