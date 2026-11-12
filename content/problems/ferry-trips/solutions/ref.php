<?php
function minTrips(array $weights, int $limit): int {
    sort($weights);
    $lo = 0;
    $hi = count($weights) - 1;
    $trips = 0;
    while ($lo <= $hi) {
        if ($weights[$lo] + $weights[$hi] <= $limit) {
            $lo++;
        }
        $hi--;
        $trips++;
    }
    return $trips;
}
