<?php
function longestRotation(array $plays, int $k): int {
    $count = [];
    $best = 0;
    $start = 0;
    foreach ($plays as $end => $song) {
        $count[$song] = ($count[$song] ?? 0) + 1;
        while (count($count) > $k) {
            $s = $plays[$start++];
            if (--$count[$s] === 0) {
                unset($count[$s]);
            }
        }
        $best = max($best, $end - $start + 1);
    }
    return $best;
}
