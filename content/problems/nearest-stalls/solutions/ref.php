<?php
function nearestStalls(array $stalls, int $k): array {
    // SplMaxHeap compares [distance, index] arrays element by element.
    $heap = new SplMaxHeap();
    foreach ($stalls as $i => [$x, $y]) {
        $heap->insert([$x * $x + $y * $y, $i]);
        if ($heap->count() > $k) {
            $heap->extract();
        }
    }
    $best = iterator_to_array($heap, false);
    usort($best, fn ($a, $b) => $a[0] <=> $b[0] ?: $a[1] <=> $b[1]);
    return array_map(fn ($e) => $e[1], $best);
}
