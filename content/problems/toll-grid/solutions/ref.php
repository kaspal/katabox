<?php
function cheapestCrossing(array $tolls): int {
    $rows = count($tolls);
    $cols = count($tolls[0]);
    $cells = $rows * $cols;
    // State s = used * cells + r * cols + c.
    $cost = array_fill(0, 2 * $cells, PHP_INT_MAX);
    $heap = new SplPriorityQueue();
    $heap->setExtractFlags(SplPriorityQueue::EXTR_BOTH);
    $start = $tolls[0][0];
    $cost[0] = $start;
    $cost[$cells] = intdiv($start, 2);
    // SplPriorityQueue pops the highest priority first, so priorities are negated.
    $heap->insert(0, -$start);
    $heap->insert($cells, -intdiv($start, 2));
    $moves = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    while (!$heap->isEmpty()) {
        $top = $heap->extract();
        $s = $top['data'];
        $d = -$top['priority'];
        if ($d > $cost[$s]) {
            continue;
        }
        $used = $s >= $cells ? 1 : 0;
        $cell = $s - $used * $cells;
        $r = intdiv($cell, $cols);
        $c = $cell % $cols;
        foreach ($moves as [$dr, $dc]) {
            $nr = $r + $dr;
            $nc = $c + $dc;
            if ($nr < 0 || $nr >= $rows || $nc < 0 || $nc >= $cols) {
                continue;
            }
            $toll = $tolls[$nr][$nc];
            $t = $used * $cells + $nr * $cols + $nc;
            if ($d + $toll < $cost[$t]) {
                $cost[$t] = $d + $toll;
                $heap->insert($t, -($d + $toll));
            }
            if (!$used) {
                $u = $cells + $nr * $cols + $nc;
                $half = $d + intdiv($toll, 2);
                if ($half < $cost[$u]) {
                    $cost[$u] = $half;
                    $heap->insert($u, -$half);
                }
            }
        }
    }
    return min($cost[$cells - 1], $cost[2 * $cells - 1]);
}
