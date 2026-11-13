<?php
function mostTalks(array $talks): int {
    usort($talks, fn ($a, $b) => $a[1] <=> $b[1]);
    $count = 0;
    $free = -1;
    foreach ($talks as [$start, $end]) {
        if ($start >= $free) {
            $count++;
            $free = $end;
        }
    }
    return $count;
}
