<?php
function windowSwings(array $readings, int $k): array {
    $maxq = new SplDoublyLinkedList();
    $minq = new SplDoublyLinkedList();
    $out = [];
    foreach ($readings as $i => $r) {
        while (!$maxq->isEmpty() && $readings[$maxq->top()] <= $r) {
            $maxq->pop();
        }
        $maxq->push($i);
        while (!$minq->isEmpty() && $readings[$minq->top()] >= $r) {
            $minq->pop();
        }
        $minq->push($i);
        if ($maxq->bottom() <= $i - $k) {
            $maxq->shift();
        }
        if ($minq->bottom() <= $i - $k) {
            $minq->shift();
        }
        if ($i >= $k - 1) {
            $out[] = $readings[$maxq->bottom()] - $readings[$minq->bottom()];
        }
    }
    return $out;
}
