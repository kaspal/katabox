<?php
function fleetTotals(array $miles, array $queries): array {
    $rows = count($miles);
    $cols = count($miles[0]);
    $p = array_fill(0, $rows + 1, array_fill(0, $cols + 1, 0));
    for ($r = 0; $r < $rows; $r++) {
        for ($c = 0; $c < $cols; $c++) {
            $p[$r + 1][$c + 1] = $miles[$r][$c] + $p[$r][$c + 1] + $p[$r + 1][$c] - $p[$r][$c];
        }
    }
    $out = [];
    foreach ($queries as [$v1, $d1, $v2, $d2]) {
        $out[] = $p[$v2 + 1][$d2 + 1] - $p[$v1][$d2 + 1] - $p[$v2 + 1][$d1] + $p[$v1][$d1];
    }
    return $out;
}
