<?php
function trappedRainGrid(array $heights): int {
    $rows = count($heights);
    $cols = count($heights[0]);
    $seen = array_fill(0, $rows * $cols, false);
    $heap = new SplMinHeap(); // [level, cell]
    for ($r = 0; $r < $rows; $r++) {
        for ($c = 0; $c < $cols; $c++) {
            if ($r === 0 || $r === $rows - 1 || $c === 0 || $c === $cols - 1) {
                $seen[$r * $cols + $c] = true;
                $heap->insert([$heights[$r][$c], $r * $cols + $c]);
            }
        }
    }
    $water = 0;
    while (!$heap->isEmpty()) {
        [$level, $cell] = $heap->extract();
        $r = intdiv($cell, $cols);
        $c = $cell % $cols;
        foreach ([[$r + 1, $c], [$r - 1, $c], [$r, $c + 1], [$r, $c - 1]] as [$nr, $nc]) {
            if ($nr < 0 || $nr >= $rows || $nc < 0 || $nc >= $cols || $seen[$nr * $cols + $nc]) {
                continue;
            }
            $seen[$nr * $cols + $nc] = true;
            $h = $heights[$nr][$nc];
            $water += max(0, $level - $h);
            $heap->insert([max($level, $h), $nr * $cols + $nc]);
        }
    }
    return $water;
}
