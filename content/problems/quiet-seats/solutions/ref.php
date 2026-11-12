<?php
function quietSeats(array $rows): int {
    $h = count($rows);
    $w = strlen($rows[0]);
    $count = 0;
    for ($r = 0; $r < $h; $r++) {
        for ($c = 0; $c < $w; $c++) {
            if ($rows[$r][$c] !== '.') {
                continue;
            }
            $quiet = true;
            for ($i = max($r - 1, 0); $i <= min($r + 1, $h - 1) && $quiet; $i++) {
                for ($j = max($c - 1, 0); $j <= min($c + 1, $w - 1); $j++) {
                    if ($rows[$i][$j] === '#') {
                        $quiet = false;
                        break;
                    }
                }
            }
            if ($quiet) {
                $count++;
            }
        }
    }
    return $count;
}
