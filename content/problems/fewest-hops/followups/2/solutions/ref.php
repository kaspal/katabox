<?php
function hopsOneWay(int $n, array $links): int {
    $fwd = array_fill(0, $n, []);
    $back = array_fill(0, $n, []);
    foreach ($links as [$a, $b]) {
        $fwd[$a][] = $b;
        $back[$b][] = $a;
    }
    $dist = array_fill(0, 2 * $n, -1);
    $dist[0] = 0;
    $q = [0];
    for ($h = 0; $h < count($q); $h++) {
        $s = $q[$h];
        $u = $s >> 1;
        $f = $s & 1;
        $d = $dist[$s];
        foreach ($fwd[$u] as $v) {
            if ($dist[$v * 2 + $f] === -1) {
                $dist[$v * 2 + $f] = $d + 1;
                $q[] = $v * 2 + $f;
            }
        }
        if ($f === 0) {
            foreach ($back[$u] as $v) {
                if ($dist[$v * 2 + 1] === -1) {
                    $dist[$v * 2 + 1] = $d + 1;
                    $q[] = $v * 2 + 1;
                }
            }
        }
    }
    $a = $dist[($n - 1) * 2];
    $b = $dist[($n - 1) * 2 + 1];
    if ($a === -1) {
        return $b;
    }
    if ($b === -1) {
        return $a;
    }
    return min($a, $b);
}
