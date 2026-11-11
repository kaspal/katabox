<?php
function longestStretch(array $costs, int $budget): int {
    $best = 0;
    $left = 0;
    $total = 0;
    $n = count($costs);
    for ($right = 0; $right < $n; $right++) {
        $total += $costs[$right];
        while ($total > $budget) {
            $total -= $costs[$left];
            $left++;
        }
        $best = max($best, $right - $left + 1);
    }
    return $best;
}
