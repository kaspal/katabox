<?php
function freshWithEncores(array $plays): int {
    $count = [];
    $best = 0;
    $start = 0;
    foreach ($plays as $end => $song) {
        $count[$song] = ($count[$song] ?? 0) + 1;
        while ($count[$song] > 2) {
            $count[$plays[$start]]--;
            $start++;
        }
        $best = max($best, $end - $start + 1);
    }
    return $best;
}
