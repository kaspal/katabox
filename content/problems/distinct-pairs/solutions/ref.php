<?php
function distinctPairs(array $prices, int $budget): int {
    $i = 0;
    $j = count($prices) - 1;
    $count = 0;
    while ($i < $j) {
        $s = $prices[$i] + $prices[$j];
        if ($s < $budget) {
            $i++;
        } elseif ($s > $budget) {
            $j--;
        } else {
            $count++;
            $left = $prices[$i];
            $right = $prices[$j];
            while ($i < $j && $prices[$i] === $left) {
                $i++;
            }
            while ($i < $j && $prices[$j] === $right) {
                $j--;
            }
        }
    }
    return $count;
}
