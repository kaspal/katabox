<?php
function mostApples(array $grid): int {
    $cols = count($grid[0]);
    $best = array_fill(0, $cols, 0);
    foreach ($grid as $r => $row) {
        for ($c = 0; $c < $cols; $c++) {
            if ($r === 0) {
                $prev = $c > 0 ? $best[$c - 1] : 0;
            } elseif ($c === 0) {
                $prev = $best[0];
            } else {
                $prev = max($best[$c], $best[$c - 1]);
            }
            $best[$c] = $row[$c] + $prev;
        }
    }
    return $best[$cols - 1];
}
