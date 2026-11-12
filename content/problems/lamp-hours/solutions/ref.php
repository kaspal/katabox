<?php
function litTime(array $lamps): int {
    usort($lamps, fn ($a, $b) => $a[0] <=> $b[0] ?: $a[1] <=> $b[1]);
    [$s, $e] = $lamps[0];
    $total = 0;
    $n = count($lamps);
    for ($i = 1; $i < $n; $i++) {
        [$start, $end] = $lamps[$i];
        if ($start <= $e) {
            $e = max($e, $end);
        } else {
            $total += $e - $s;
            $s = $start;
            $e = $end;
        }
    }
    return $total + $e - $s;
}
