<?php
function longestRun(array $values): int {
    $best = 0;
    $run = 1;
    for ($i = 1; $i < count($values); $i++) {
        if ($values[$i] === $values[$i - 1]) {
            $run++;
        } else {
            $best = max($best, $run);
            $run = 1;
        }
    }
    return $best;
}
