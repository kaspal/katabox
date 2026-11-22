<?php
function bothBusyMerge(array $bookings): array {
    usort($bookings, fn($x, $y) => [$x[0], $x[1]] <=> [$y[0], $y[1]]);
    $out = [];
    foreach ($bookings as [$s, $e]) {
        $k = count($out) - 1;
        if ($k >= 0 && $s <= $out[$k][1]) {
            $out[$k][1] = max($out[$k][1], $e);
        } else {
            $out[] = [$s, $e];
        }
    }
    return $out;
}

function bothBusy(array $roomA, array $roomB): array {
    $a = bothBusyMerge($roomA);
    $b = bothBusyMerge($roomB);
    $out = [];
    $i = 0;
    $j = 0;
    while ($i < count($a) && $j < count($b)) {
        $s = max($a[$i][0], $b[$j][0]);
        $e = min($a[$i][1], $b[$j][1]);
        if ($s <= $e) {
            $out[] = [$s, $e];
        }
        if ($a[$i][1] < $b[$j][1]) {
            $i++;
        } else {
            $j++;
        }
    }
    return $out;
}
