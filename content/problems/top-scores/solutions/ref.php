<?php
function topScores(array $scores, int $k): array {
    $heap = new SplMinHeap();
    foreach ($scores as $s) {
        if ($heap->count() < $k) {
            $heap->insert($s);
        } elseif ($s > $heap->top()) {
            $heap->extract();
            $heap->insert($s);
        }
    }
    $out = [];
    foreach ($heap as $s) {
        $out[] = $s;
    }
    rsort($out);
    return $out;
}
