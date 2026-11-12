<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function rotatePlaylist(?ListNode $playlist, int $k): ?ListNode {
    if ($playlist === null) {
        return null;
    }
    $n = 1;
    $tail = $playlist;
    while ($tail->next !== null) {
        $tail = $tail->next;
        $n++;
    }
    $r = $k % $n;
    if ($r === 0) {
        return $playlist;
    }
    $newTail = $playlist;
    for ($i = 0; $i < $n - $r - 1; $i++) {
        $newTail = $newTail->next;
    }
    $head = $newTail->next;
    $newTail->next = null;
    $tail->next = $playlist;
    return $head;
}
