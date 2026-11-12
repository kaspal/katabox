<?php
function turnRight(array $grid): array {
    $r = count($grid);
    $c = count($grid[0]);
    $out = [];
    for ($j = 0; $j < $c; $j++) {
        $row = [];
        for ($i = $r - 1; $i >= 0; $i--) {
            $row[] = $grid[$i][$j];
        }
        $out[] = $row;
    }
    return $out;
}
