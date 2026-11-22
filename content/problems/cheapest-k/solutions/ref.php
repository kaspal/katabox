<?php
function cheapestK(array $prices, int $k): int {
    $heap = new SplMaxHeap();
    foreach ($prices as $p) {
        $heap->insert($p);
        if (count($heap) > $k) {
            $heap->extract();
        }
    }
    $total = 0;
    foreach ($heap as $p) {
        $total += $p;
    }
    return $total;
}
