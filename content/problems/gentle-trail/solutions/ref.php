<?php
function gentlestClimb(array $heights): int {
    $rows = count($heights);
    $cols = count($heights[0]);
    $n = $rows * $cols;
    $flat = array_merge(...$heights);
    $best = array_fill(0, $n, PHP_INT_MAX);
    $best[0] = 0;
    // SplPriorityQueue is a max-heap: use negated efforts as priorities.
    $heap = new SplPriorityQueue();
    $heap->setExtractFlags(SplPriorityQueue::EXTR_BOTH);
    $heap->insert(0, 0);
    while (!$heap->isEmpty()) {
        ['data' => $i, 'priority' => $p] = $heap->extract();
        $d = -$p;
        if ($i === $n - 1) {
            return $d;
        }
        if ($d > $best[$i]) {
            continue;
        }
        $r = intdiv($i, $cols);
        $c = $i - $r * $cols;
        $h = $flat[$i];
        $next = [];
        if ($r > 0) {
            $next[] = $i - $cols;
        }
        if ($r + 1 < $rows) {
            $next[] = $i + $cols;
        }
        if ($c > 0) {
            $next[] = $i - 1;
        }
        if ($c + 1 < $cols) {
            $next[] = $i + 1;
        }
        foreach ($next as $j) {
            $nd = max($d, abs($flat[$j] - $h));
            if ($nd < $best[$j]) {
                $best[$j] = $nd;
                $heap->insert($j, -$nd);
            }
        }
    }
    return 0;
}
