<?php
function kinshipDistances(array $parent, array $pairs): array {
    $n = count($parent);
    $children = array_fill(0, $n, []);
    $root = 0;
    foreach ($parent as $i => $p) {
        if ($p === -1) {
            $root = $i;
        } else {
            $children[$p][] = $i;
        }
    }
    $depth = array_fill(0, $n, 0);
    $order = [$root];
    for ($h = 0; $h < count($order); $h++) {
        foreach ($children[$order[$h]] as $v) {
            $depth[$v] = $depth[$order[$h]] + 1;
            $order[] = $v;
        }
    }
    $levels = 1;
    while ((1 << $levels) < $n) {
        $levels++;
    }
    $up = [array_map(fn($p) => $p === -1 ? $root : $p, $parent)];
    for ($k = 1; $k < $levels; $k++) {
        $prev = $up[$k - 1];
        $row = [];
        foreach ($prev as $v => $p) {
            $row[$v] = $prev[$p];
        }
        $up[] = $row;
    }
    $out = [];
    foreach ($pairs as [$a, $b]) {
        $x = $depth[$a] >= $depth[$b] ? $a : $b;
        $y = $x === $a ? $b : $a;
        $steps = $depth[$x] - $depth[$y];
        for ($k = 0; $steps > 0; $k++, $steps >>= 1) {
            if ($steps & 1) {
                $x = $up[$k][$x];
            }
        }
        if ($x !== $y) {
            for ($k = $levels - 1; $k >= 0; $k--) {
                if ($up[$k][$x] !== $up[$k][$y]) {
                    $x = $up[$k][$x];
                    $y = $up[$k][$y];
                }
            }
            $x = $up[0][$x];
        }
        $out[] = $depth[$a] + $depth[$b] - 2 * $depth[$x];
    }
    return $out;
}
