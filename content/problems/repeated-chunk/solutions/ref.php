<?php
function longestRepeat(string $text): int {
    // Two primes below 2^26, as in the other references.
    $m1 = 50331653;
    $m2 = 25165843;
    $b = 131;
    $n = strlen($text);
    $h1 = [0];
    $h2 = [0];
    $p1 = [1];
    $p2 = [1];
    for ($i = 0; $i < $n; $i++) {
        $c = ord($text[$i]);
        $h1[] = ($h1[$i] * $b + $c) % $m1;
        $h2[] = ($h2[$i] * $b + $c) % $m2;
        $p1[] = $p1[$i] * $b % $m1;
        $p2[] = $p2[$i] * $b % $m2;
    }
    $repeats = function (int $len) use ($n, $h1, $h2, $p1, $p2, $m1, $m2): bool {
        $seen = [];
        for ($i = 0; $i + $len <= $n; $i++) {
            $a = (($h1[$i + $len] - $h1[$i] * $p1[$len]) % $m1 + $m1) % $m1;
            $c = (($h2[$i + $len] - $h2[$i] * $p2[$len]) % $m2 + $m2) % $m2;
            $key = $a * $m2 + $c;
            if (isset($seen[$key])) {
                return true;
            }
            $seen[$key] = true;
        }
        return false;
    };
    $lo = 0;
    $hi = $n - 1;
    while ($lo < $hi) {
        $mid = intdiv($lo + $hi + 1, 2);
        if ($repeats($mid)) {
            $lo = $mid;
        } else {
            $hi = $mid - 1;
        }
    }
    return $lo;
}
