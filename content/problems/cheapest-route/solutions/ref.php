<?php
function cheapestRoute(int $n, array $roads, int $start, int $goal): int {
    $adj = array_fill(0, $n, []);
    foreach ($roads as [$a, $b, $toll]) {
        $adj[$a][] = [$b, $toll];
        $adj[$b][] = [$a, $toll];
    }
    $dist = array_fill(0, $n, PHP_INT_MAX);
    $dist[$start] = 0;
    $heap = new SplPriorityQueue();
    $heap->setExtractFlags(SplPriorityQueue::EXTR_BOTH);
    $heap->insert($start, 0); // priorities are negated costs (max-heap)
    while (!$heap->isEmpty()) {
        $top = $heap->extract();
        $t = $top['data'];
        $d = -$top['priority'];
        if ($d > $dist[$t]) {
            continue;
        }
        if ($t === $goal) {
            return $d;
        }
        foreach ($adj[$t] as [$u, $toll]) {
            $nd = $d + $toll;
            if ($nd < $dist[$u]) {
                $dist[$u] = $nd;
                $heap->insert($u, -$nd);
            }
        }
    }
    return -1;
}
