<?php
function mergeBookings(array $bookings): array {
    usort($bookings, fn ($a, $b) => $a[0] <=> $b[0] ?: $a[1] <=> $b[1]);
    $out = [];
    foreach ($bookings as [$s, $e]) {
        $last = count($out) - 1;
        if ($last >= 0 && $s <= $out[$last][1]) {
            $out[$last][1] = max($out[$last][1], $e);
        } else {
            $out[] = [$s, $e];
        }
    }
    return $out;
}
