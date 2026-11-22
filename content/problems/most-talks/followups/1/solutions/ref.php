<?php
function bestProgramme(array $talks): int {
    usort($talks, fn($a, $b) => $a[1] <=> $b[1]);
    $n = count($talks);
    $best = array_fill(0, $n + 1, 0);
    for ($i = 0; $i < $n; $i++) {
        [$s, , $score] = $talks[$i];
        $lo = 0;
        $hi = $i;
        while ($lo < $hi) {
            $m = intdiv($lo + $hi, 2);
            if ($talks[$m][1] <= $s) {
                $lo = $m + 1;
            } else {
                $hi = $m;
            }
        }
        $best[$i + 1] = max($best[$i], $best[$lo] + $score);
    }
    return $best[$n];
}
