<?php
function biggestTank(array $boards): int {
    $i = 0;
    $j = count($boards) - 1;
    $best = 0;
    while ($i < $j) {
        if ($boards[$i] <= $boards[$j]) {
            $best = max($best, ($j - $i) * $boards[$i]);
            $i++;
        } else {
            $best = max($best, ($j - $i) * $boards[$j]);
            $j--;
        }
    }
    return $best;
}
