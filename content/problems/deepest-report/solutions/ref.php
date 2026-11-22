<?php
function deepestReport(array $manager): int {
    $n = count($manager);
    $depth = array_fill(0, $n, 0);
    $best = 0;
    for ($v = 0; $v < $n; $v++) {
        $path = [];
        $u = $v;
        while ($u !== -1 && $depth[$u] === 0) {
            $path[] = $u;
            $u = $manager[$u];
        }
        $d = $u === -1 ? 0 : $depth[$u];
        for ($i = count($path) - 1; $i >= 0; $i--) {
            $depth[$path[$i]] = ++$d;
        }
        $best = max($best, $depth[$v]);
    }
    return $best;
}
