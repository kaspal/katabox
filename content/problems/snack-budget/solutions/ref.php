<?php
function mostSnacks(array $prices, int $budget): int {
    sort($prices);
    $spent = 0;
    $count = 0;
    foreach ($prices as $p) {
        if ($spent + $p > $budget) {
            break;
        }
        $spent += $p;
        $count++;
    }
    return $count;
}
