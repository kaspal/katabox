<?php
function minRooms(array $meetings): int {
    $starts = array_column($meetings, 0);
    $ends = array_column($meetings, 1);
    sort($starts);
    sort($ends);
    $rooms = 0;
    $best = 0;
    $j = 0;
    foreach ($starts as $s) {
        while ($ends[$j] <= $s) {
            $rooms--;
            $j++;
        }
        $rooms++;
        $best = max($best, $rooms);
    }
    return $best;
}
