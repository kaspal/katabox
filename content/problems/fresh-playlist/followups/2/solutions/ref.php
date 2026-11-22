<?php
function fewestBlocks(array $plays, array $requests): array {
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
    $next = array_fill(0, $n + 1, $n);
    $r = 0;
    for ($i = 0; $i < $n; $i++) {
        while ($r < $n && $start[$r] <= $i) {
            $r++;
        }
        $next[$i] = $r;
    }
    $jump = [$next];
    while ((1 << count($jump)) <= $n) {
        $prev = $jump[count($jump) - 1];
        $row = [];
        for ($i = 0; $i <= $n; $i++) {
            $row[] = $prev[$prev[$i]];
        }
        $jump[] = $row;
    }
    $out = [];
    foreach ($requests as [$a, $b]) {
        $pos = $a;
        $blocks = 1;
        for ($k = count($jump) - 1; $k >= 0; $k--) {
            if ($jump[$k][$pos] <= $b) {
                $pos = $jump[$k][$pos];
                $blocks += 1 << $k;
            }
        }
        $out[] = $blocks;
    }
    return $out;
}
