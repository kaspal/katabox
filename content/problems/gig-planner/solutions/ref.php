<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$gigs = [];
for ($i = 0; $i < $n; $i++) {
    $gigs[] = [(int) $tokens[2 + 3 * $i], (int) $tokens[1 + 3 * $i], (int) $tokens[3 + 3 * $i]]; // [end, start, pay]
}
usort($gigs, fn($a, $b) => $a[0] <=> $b[0]);
$ends = array_column($gigs, 0);
// best[i]: the best plan using only the first i offers (by end): a higher total wins,
// then fewer gigs. Either offer i is skipped, or it is taken after the best plan of the
// offers that end by its start (a binary search).
$total = array_fill(0, $n + 1, 0);
$count = array_fill(0, $n + 1, 0);
for ($i = 0; $i < $n; $i++) {
    [, $start, $pay] = $gigs[$i];
    $lo = 0;
    $hi = $i;
    while ($lo < $hi) {
        $mid = ($lo + $hi) >> 1;
        if ($ends[$mid] <= $start) {
            $lo = $mid + 1;
        } else {
            $hi = $mid;
        }
    }
    $t = $total[$lo] + $pay;
    $c = $count[$lo] + 1;
    if ($t > $total[$i] || ($t === $total[$i] && $c < $count[$i])) {
        $total[$i + 1] = $t;
        $count[$i + 1] = $c;
    } else {
        $total[$i + 1] = $total[$i];
        $count[$i + 1] = $count[$i];
    }
}
echo $total[$n], ' ', $count[$n], "\n";
