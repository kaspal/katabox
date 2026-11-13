<?php
function powerMod(int $base, int $exponent): int {
    $p = 1000003;
    $result = 1;
    $square = $base % $p;
    while ($exponent > 0) {
        if ($exponent % 2 === 1) {
            $result = $result * $square % $p;
        }
        $square = $square * $square % $p;
        $exponent = intdiv($exponent, 2);
    }
    return $result;
}
