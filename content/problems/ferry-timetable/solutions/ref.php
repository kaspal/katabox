<?php
function ferryTimes(int $n, array $routes, array $queries): array {
    $inf = PHP_INT_MAX;
    $dist = array_fill(0, $n, array_fill(0, $n, $inf));
    for ($i = 0; $i < $n; $i++) {
        $dist[$i][$i] = 0;
    }
    foreach ($routes as [$a, $b, $w]) {
        if ($w < $dist[$a][$b]) {
            $dist[$a][$b] = $w;
            $dist[$b][$a] = $w;
        }
    }
    for ($k = 0; $k < $n; $k++) {
        for ($i = 0; $i < $n; $i++) {
            $dik = $dist[$i][$k];
            if ($dik === $inf) {
                continue;
            }
            for ($j = 0; $j < $n; $j++) {
                $dkj = $dist[$k][$j];
                if ($dkj !== $inf && $dik + $dkj < $dist[$i][$j]) {
                    $dist[$i][$j] = $dik + $dkj;
                }
            }
        }
    }
    $out = [];
    foreach ($queries as [$a, $b]) {
        $out[] = $dist[$a][$b] === $inf ? -1 : $dist[$a][$b];
    }
    return $out;
}
