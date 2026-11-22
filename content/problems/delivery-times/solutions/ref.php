<?php
function deliveryTimes(int $n, array $roads, int $start): array {
    $adj = array_fill(0, $n, []);
    foreach ($roads as [$a, $b, $w]) {
        $adj[$a][] = [$b, $w];
        $adj[$b][] = [$a, $w];
    }
    $dist = array_fill(0, $n, -1);
    $heap = new SplPriorityQueue();
    $heap->setExtractFlags(SplPriorityQueue::EXTR_BOTH);
    $heap->insert($start, 0);
    while (!$heap->isEmpty()) {
        $top = $heap->extract();
        $v = $top['data'];
        $d = -$top['priority'];
        if ($dist[$v] !== -1) {
            continue;
        }
        $dist[$v] = $d;
        foreach ($adj[$v] as [$u, $w]) {
            if ($dist[$u] === -1) {
                $heap->insert($u, -($d + $w));
            }
        }
    }
    return $dist;
}
