<?php
// The remainder in [0, m), also for negative v.
function modulo(int $v, int $m): int {
    return (($v % $m) + $m) % $m;
}

function gcd(int $a, int $b): int {
    while ($b !== 0) {
        [$a, $b] = [$b, $a % $b];
    }
    return $a;
}

// a⁻¹ modulo m (gcd(a, m) = 1) by the extended Euclidean algorithm; 0 when m = 1.
function inverse(int $a, int $m): int {
    [$oldR, $r] = [$a, $m];
    [$oldS, $s] = [1, 0];
    while ($r !== 0) {
        $q = intdiv($oldR, $r);
        [$oldR, $r] = [$r, $oldR - $q * $r];
        [$oldS, $s] = [$s, $oldS - $q * $s];
    }
    return modulo($oldS, $m);
}

$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$t = (int) $tokens[$pos++];
$out = [];
for ($c = 0; $c < $t; $c++) {
    $n = (int) $tokens[$pos++];
    $x = 0; // the lines so far all leave at minutes x + big·k
    $big = 1;
    $ok = true;
    for ($i = 0; $i < $n; $i++) {
        $r = (int) $tokens[$pos++];
        $m = (int) $tokens[$pos++];
        if (!$ok) {
            continue;
        }
        $g = gcd($big, $m);
        if (($r - $x) % $g !== 0) {
            $ok = false;
            continue;
        }
        $step = intdiv($m, $g);
        $k = modulo(intdiv($r - $x, $g), $step) * inverse(modulo(intdiv($big, $g), $step), $step) % $step;
        $x += $big * $k;
        $big *= $step;
    }
    $out[] = $ok ? $x : -1;
}
echo implode("\n", $out), "\n";
