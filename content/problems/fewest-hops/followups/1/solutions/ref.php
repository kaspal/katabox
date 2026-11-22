<?php
function hopsToAll(int $n, array $links): array {
    $adj = array_fill(0, $n, []);
    foreach ($links as [$a, $b]) {
        $adj[$a][] = $b;
        $adj[$b][] = $a;
    }
    $dist = array_fill(0, $n, -1);
    $dist[0] = 0;
    $q = [0];
    for ($h = 0; $h < count($q); $h++) {
        $u = $q[$h];
        foreach ($adj[$u] as $v) {
            if ($dist[$v] === -1) {
                $dist[$v] = $dist[$u] + 1;
                $q[] = $v;
            }
        }
    }
    return $dist;
}
