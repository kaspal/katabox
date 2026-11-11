<?php
function bestPair(array $prices, int $budget): int {
    $lo = 0;
    $hi = count($prices) - 1;
    $best = -1;
    while ($lo < $hi) {
        $total = $prices[$lo] + $prices[$hi];
        if ($total > $budget) {
            $hi--;
        } else {
            $best = max($best, $total);
            $lo++;
        }
    }
    return $best;
}
