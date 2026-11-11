<?php
function shortestRoute(array $grid): int {
    $rows = count($grid);
    $cols = count($grid[0]);
    if ($grid[0][0] === 1 || $grid[$rows - 1][$cols - 1] === 1) {
        return -1;
    }
    $dist = array_fill(0, $rows * $cols, -1);
    $queue = [0];
    $dist[0] = 0;
    $head = 0;
    $moves = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    while ($head < count($queue)) {
        $cell = $queue[$head++];
        $r = intdiv($cell, $cols);
        $c = $cell % $cols;
        if ($r === $rows - 1 && $c === $cols - 1) {
            return $dist[$cell];
        }
        foreach ($moves as [$dr, $dc]) {
            $nr = $r + $dr;
            $nc = $c + $dc;
            if ($nr < 0 || $nr >= $rows || $nc < 0 || $nc >= $cols || $grid[$nr][$nc] !== 0) {
                continue;
            }
            $next = $nr * $cols + $nc;
            if ($dist[$next] !== -1) {
                continue;
            }
            $dist[$next] = $dist[$cell] + 1;
            $queue[] = $next;
        }
    }
    return -1;
}
