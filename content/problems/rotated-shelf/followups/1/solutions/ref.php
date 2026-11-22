<?php
function findLabels(array $shelf, array $labels): array {
    $n = count($shelf);
    $lo = 0;
    $hi = $n - 1;
    while ($lo < $hi) {
        $m = intdiv($lo + $hi, 2);
        if ($shelf[$m] > $shelf[$hi]) {
            $lo = $m + 1;
        } else {
            $hi = $m;
        }
    }
    $start = $lo;
    $out = [];
    foreach ($labels as $x) {
        $a = 0;
        $b = $n;
        while ($a < $b) {
            $m = intdiv($a + $b, 2);
            if ($shelf[($m + $start) % $n] < $x) {
                $a = $m + 1;
            } else {
                $b = $m;
            }
        }
        $out[] = ($a < $n && $shelf[($a + $start) % $n] === $x) ? ($a + $start) % $n : -1;
    }
    return $out;
}
