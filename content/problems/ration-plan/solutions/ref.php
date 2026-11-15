<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$n = (int) $tokens[$pos++];
$q = (int) $tokens[$pos++];
$prices = array_map('intval', array_slice($tokens, $pos, $n));
$pos += $n;
sort($prices);
// total[k - 1]: the cost of the k cheapest rations.
$total = [];
$sum = 0;
foreach ($prices as $p) {
    $sum += $p;
    $total[] = $sum;
}
$out = [];
for ($i = 0; $i < $q; $i++) {
    $budget = (int) $tokens[$pos++];
    // The number of totals within the budget: the first index whose total exceeds it.
    $lo = 0;
    $hi = $n;
    while ($lo < $hi) {
        $mid = ($lo + $hi) >> 1;
        if ($total[$mid] <= $budget) {
            $lo = $mid + 1;
        } else {
            $hi = $mid;
        }
    }
    $out[] = $lo;
}
echo implode("\n", $out), "\n";
