<?php
function blockedWalks(int $rows, int $cols, array $blocked): int {
    $p = 1000003;
    $top = $rows + $cols;
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
    usort($blocked, fn ($x, $y) => [$x[0], $x[1]] <=> [$y[0], $y[1]]);
    $blocked[] = [$rows, $cols];
    $good = [];
    foreach ($blocked as $i => [$pr, $pc]) {
        $ways = $fact[$pr + $pc] * $inv[$pr] % $p * $inv[$pc] % $p;
        for ($j = 0; $j < $i; $j++) {
            [$qr, $qc] = $blocked[$j];
            if ($qr <= $pr && $qc <= $pc) {
                $dr = $pr - $qr;
                $dc = $pc - $qc;
                $ways -= $good[$j] * ($fact[$dr + $dc] * $inv[$dr] % $p * $inv[$dc] % $p) % $p;
            }
        }
        $good[] = (($ways % $p) + $p) % $p;
    }
    return $good[count($good) - 1];
}
