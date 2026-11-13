<?php
// Provided by KataBox: the judge builds the input lists with this class.
class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function splitTrain(?ListNode $train, int $k): array {
    $n = 0;
    for ($node = $train; $node !== null; $node = $node->next) {
        $n++;
    }
    $base = intdiv($n, $k);
    $extra = $n % $k;
    $parts = [];
    $node = $train;
    for ($i = 0; $i < $k; $i++) {
        $size = $base + ($i < $extra ? 1 : 0);
        if ($size === 0) {
            $parts[] = null;
            continue;
        }
        $parts[] = $node;
        for ($j = 0; $j < $size - 1; $j++) {
            $node = $node->next;
        }
        $next = $node->next;
        $node->next = null;
        $node = $next;
    }
    return $parts;
}
