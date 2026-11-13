<?php
function plotTotals(array $grid, array $queries): array {
    $cols = count($grid[0]);
    $pre = [array_fill(0, $cols + 1, 0)];
    foreach ($grid as $r => $row) {
        $above = $pre[$r];
        $cur = [0];
        $run = 0;
        for ($c = 0; $c < $cols; $c++) {
            $run += $row[$c];
            $cur[] = $above[$c + 1] + $run;
        }
        $pre[] = $cur;
    }
    $out = [];
    foreach ($queries as [$t, $l, $b, $r]) {
        $out[] = $pre[$b + 1][$r + 1] - $pre[$t][$r + 1] - $pre[$b + 1][$l] + $pre[$t][$l];
    }
    return $out;
}
