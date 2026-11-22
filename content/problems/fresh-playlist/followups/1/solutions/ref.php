<?php
function freshInRange(array $plays, array $requests): array {
    $n = count($plays);
    $start = array_fill(0, $n, 0);
    $last = [];
    $left = 0;
    for ($r = 0; $r < $n; $r++) {
        $song = $plays[$r];
        if (isset($last[$song]) && $last[$song] >= $left) {
            $left = $last[$song] + 1;
        }
        $last[$song] = $r;
        $start[$r] = $left;
    }
    $table = [[]];
    for ($r = 0; $r < $n; $r++) {
        $table[0][] = $r - $start[$r] + 1;
    }
    for ($j = 1; (1 << $j) <= $n; $j++) {
        $prev = $table[$j - 1];
        $half = 1 << ($j - 1);
        $row = [];
        $len = $n - (1 << $j) + 1;
        for ($i = 0; $i < $len; $i++) {
            $row[] = max($prev[$i], $prev[$i + $half]);
        }
        $table[] = $row;
    }
    $out = [];
    foreach ($requests as [$a, $b]) {
        $lo = $a;
        $hi = $b + 1;
        while ($lo < $hi) {
            $mid = intdiv($lo + $hi, 2);
            if ($start[$mid] >= $a) {
                $hi = $mid;
            } else {
                $lo = $mid + 1;
            }
        }
        $best = $lo - $a;
        if ($lo <= $b) {
            $k = strlen(decbin($b - $lo + 1)) - 1;
            $best = max($best, $table[$k][$lo], $table[$k][$b - (1 << $k) + 1]);
        }
        $out[] = $best;
    }
    return $out;
}
