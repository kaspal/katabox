<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function zigzagBoarding(?ListNode $queue): ?ListNode {
    if ($queue === null || $queue->next === null) {
        return $queue;
    }
    $slow = $queue;
    $fast = $queue->next;
    while ($fast !== null && $fast->next !== null) {
        $slow = $slow->next;
        $fast = $fast->next->next;
    }
    $back = $slow->next;
    $slow->next = null;
    $prev = null;
    while ($back !== null) {
        $next = $back->next;
        $back->next = $prev;
        $prev = $back;
        $back = $next;
    }
    $front = $queue;
    $back = $prev;
    while ($back !== null) {
        $frontNext = $front->next;
        $backNext = $back->next;
        $front->next = $back;
        $back->next = $frontNext;
        $front = $frontNext;
        $back = $backNext;
    }
    return $queue;
}
