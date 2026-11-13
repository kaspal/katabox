<?php
function freshStretch(array $plays): int {
    $last = [];
    $start = 0;
    $best = 0;
    foreach ($plays as $end => $x) {
        if (isset($last[$x]) && $last[$x] >= $start) {
            $start = $last[$x] + 1;
        }
        $last[$x] = $end;
        $best = max($best, $end - $start + 1);
    }
    return $best;
}
