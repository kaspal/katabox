<?php
function leapYears(int $start, int $end): int {
    $count = 0;
    for ($y = $start; $y <= $end; $y++) {
        if ($y % 4 === 0 && ($y % 100 !== 0 || $y % 400 === 0)) {
            $count++;
        }
    }
    return $count;
}
