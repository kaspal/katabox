<?php
function commonBoss(array $boss, array $pairs): array {
    $n = count($boss);
    $children = array_fill(0, $n, []);
    $president = 0;
    foreach ($boss as $v => $b) {
        if ($b < 0) {
            $president = $v;
        } else {
            $children[$b][] = $v;
        }
    }
    $depth = array_fill(0, $n, 0);
    $order = [$president];
    for ($h = 0; $h < count($order); $h++) {
        $v = $order[$h];
        foreach ($children[$v] as $c) {
            $depth[$c] = $depth[$v] + 1;
            $order[] = $c;
        }
    }
    $levels = 1;
    while ((1 << $levels) < $n) {
        $levels++;
    }
    $up = [array_map(fn ($b) => $b < 0 ? $president : $b, $boss)];
    for ($k = 1; $k < $levels; $k++) {
        $prev = $up[$k - 1];
        $cur = [];
        for ($v = 0; $v < $n; $v++) {
            $cur[$v] = $prev[$prev[$v]];
        }
        $up[] = $cur;
    }
    $out = [];
    foreach ($pairs as [$a, $b]) {
        if ($depth[$a] < $depth[$b]) {
            [$a, $b] = [$b, $a];
        }
        $d = $depth[$a] - $depth[$b];
        for ($k = 0; $d > 0; $k++, $d >>= 1) {
            if ($d & 1) {
                $a = $up[$k][$a];
            }
        }
        if ($a !== $b) {
            for ($k = $levels - 1; $k >= 0; $k--) {
                if ($up[$k][$a] !== $up[$k][$b]) {
                    $a = $up[$k][$a];
                    $b = $up[$k][$b];
                }
            }
            $a = $up[0][$a];
        }
        $out[] = $a;
    }
    return $out;
}
