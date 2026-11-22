<?php
function totalCovered(array $shifts): int {
    usort($shifts, fn($x, $y) => $x[0] <=> $y[0] ?: $x[1] <=> $y[1]);
    $total = 0;
    [$curS, $curE] = $shifts[0];
    foreach (array_slice($shifts, 1) as [$s, $e]) {
        if ($s <= $curE) {
            $curE = max($curE, $e);
        } else {
            $total += $curE - $curS;
            [$curS, $curE] = [$s, $e];
        }
    }
    return $total + $curE - $curS;
}
