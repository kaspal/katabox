<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$t = (int) $tokens[0];
$pos = 1;
$out = [];
for ($day = 0; $day < $t; $day++) {
    $n = (int) $tokens[$pos];
    $total = 0;
    $highest = 0;
    for ($k = 1; $k <= $n; $k++) {
        $price = (int) $tokens[$pos + $k];
        $total += $price;
        if ($price > $highest) {
            $highest = $price;
        }
    }
    $pos += 1 + $n;
    $out[] = "$n $total $highest";
}
echo implode("\n", $out), "\n";
