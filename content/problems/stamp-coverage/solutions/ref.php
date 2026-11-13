<?php
function busiestCells(int $rows, int $cols, array $stamps): array {
    $w = $cols + 1;
    $d = array_fill(0, ($rows + 1) * $w, 0);
    foreach ($stamps as [$t, $l, $b, $r]) {
        $d[$t * $w + $l]++;
        $d[$t * $w + $r + 1]--;
        $d[($b + 1) * $w + $l]--;
        $d[($b + 1) * $w + $r + 1]++;
    }
    $most = 0;
    $cells = 0;
    $above = array_fill(0, $cols, 0);
    for ($i = 0; $i < $rows; $i++) {
        $run = 0;
        $base = $i * $w;
        for ($j = 0; $j < $cols; $j++) {
            $run += $d[$base + $j];
            $v = $above[$j] + $run;
            $above[$j] = $v;
            if ($v > $most) {
                $most = $v;
                $cells = 1;
            } elseif ($v === $most) {
                $cells++;
            }
        }
    }
    return [$most, $cells];
}
