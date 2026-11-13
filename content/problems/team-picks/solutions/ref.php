<?php
function teamPicks(array $queries): array {
    $p = 1000003;
    $top = max(array_map(fn ($q) => $q[0], $queries));
    $fact = [1];
    for ($i = 1; $i <= $top; $i++) {
        $fact[$i] = $fact[$i - 1] * $i % $p;
    }
    $r = 1;
    $b = $fact[$top];
    for ($e = $p - 2; $e > 0; $e = intdiv($e, 2)) {
        if ($e % 2 === 1) {
            $r = $r * $b % $p;
        }
        $b = $b * $b % $p;
    }
    $inv = array_fill(0, $top + 1, 1);
    $inv[$top] = $r;
    for ($i = $top; $i > 0; $i--) {
        $inv[$i - 1] = $inv[$i] * $i % $p;
    }
    $out = [];
    foreach ($queries as [$n, $k]) {
        $out[] = $k > $n ? 0 : $fact[$n] * $inv[$k] % $p * $inv[$n - $k] % $p;
    }
    return $out;
}
