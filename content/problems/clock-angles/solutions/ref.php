<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$q = (int) $tokens[0];
$out = [];
for ($i = 1; $i <= $q; $i++) {
    $h = (int) substr($tokens[$i], 0, 2);
    $m = (int) substr($tokens[$i], 3);
    // In half-degrees, so everything stays whole: the hour hand is at 60·h + m,
    // the minute hand at 12·m.
    $gap = abs(60 * ($h % 12) - 11 * $m);
    $gap = min($gap, 720 - $gap);
    $out[] = intdiv($gap, 2) . ($gap % 2 ? '.5' : '.0');
}
echo implode("\n", $out), "\n";
