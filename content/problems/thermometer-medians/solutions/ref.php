<?php
function runningMedians(array $readings): array {
    $low = new SplMaxHeap();
    $high = new SplMinHeap();
    $out = [];
    foreach ($readings as $x) {
        if ($low->isEmpty() || $x <= $low->top()) {
            $low->insert($x);
        } else {
            $high->insert($x);
        }
        if ($low->count() > $high->count() + 1) {
            $high->insert($low->extract());
        } elseif ($high->count() > $low->count()) {
            $low->insert($high->extract());
        }
        $out[] = $low->count() > $high->count()
            ? (float) $low->top()
            : ($low->top() + $high->top()) / 2.0;
    }
    return $out;
}
