<?php
function cheapestWithPass(int $n, array $roads, int $start, int $goal): int {
    $adj = array_fill(0, $n, []);
    foreach ($roads as [$a, $b, $t]) {
        $adj[$a][] = [$b, $t];
        $adj[$b][] = [$a, $t];
    }
    $inf = PHP_INT_MAX;
    $dist = array_fill(0, 2 * $n, $inf);
    $dist[$start * 2] = 0;
    $heap = new SplPriorityQueue();
    $heap->setExtractFlags(SplPriorityQueue::EXTR_BOTH);
    $heap->insert($start * 2, 0);
    while (!$heap->isEmpty()) {
        $top = $heap->extract();
        $s = $top['data'];
        $d = -$top['priority'];
        if ($d > $dist[$s]) {
            continue;
        }
        $u = $s >> 1;
        $used = $s & 1;
        foreach ($adj[$u] as [$v, $t]) {
            if ($d + $t < $dist[$v * 2 + $used]) {
                $dist[$v * 2 + $used] = $d + $t;
                $heap->insert($v * 2 + $used, -($d + $t));
            }
            if ($used === 0 && $d < $dist[$v * 2 + 1]) {
                $dist[$v * 2 + 1] = $d;
                $heap->insert($v * 2 + 1, -$d);
            }
        }
    }
    $best = min($dist[$goal * 2], $dist[$goal * 2 + 1]);
    return $best === $inf ? -1 : $best;
}
