<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$t = (int) $tokens[$pos++];
$out = [];
for ($c = 0; $c < $t; $c++) {
    $n = (int) $tokens[$pos++];
    $heights = array_map('intval', array_slice($tokens, $pos, $n));
    $pos += $n;
    sort($heights);
    // The middle singer, or the lower of the two middle ones when n is even.
    $out[] = $heights[intdiv($n - 1, 2)];
}
echo implode("\n", $out), "\n";
