<?php
function trappedRain(array $columns): int {
    $i = 0;
    $j = count($columns) - 1;
    $leftMax = 0;
    $rightMax = 0;
    $water = 0;
    while ($i <= $j) {
        $leftMax = max($leftMax, $columns[$i]);
        $rightMax = max($rightMax, $columns[$j]);
        if ($leftMax <= $rightMax) {
            $water += $leftMax - $columns[$i];
            $i++;
        } else {
            $water += $rightMax - $columns[$j];
            $j--;
        }
    }
    return $water;
}
