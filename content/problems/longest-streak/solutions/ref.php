<?php
function longestRun(array $values): int {
    $best = 1;
    $run = 1;
    for ($i = 1; $i < count($values); $i++) {
        $run = $values[$i] === $values[$i - 1] ? $run + 1 : 1;
        $best = max($best, $run);
    }
    return $best;
}
