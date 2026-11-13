<?php
function roadRepairs(int $length, array $repairs): array {
    $change = array_fill(0, $length + 1, 0);
    foreach ($repairs as [$a, $b, $crews]) {
        $change[$a] += $crews;
        $change[$b + 1] -= $crews;
    }
    $out = [];
    $running = 0;
    for ($i = 0; $i < $length; $i++) {
        $running += $change[$i];
        $out[] = $running;
    }
    return $out;
}
