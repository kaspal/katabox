<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$jobs = [];
for ($i = 0; $i < $n; $i++) {
    $jobs[] = [(int) $tokens[2 + 2 * $i], (int) $tokens[1 + 2 * $i]]; // [fee, deadline]
}
rsort($jobs);
// Best fee first, each on the latest free day by its deadline (only days 1..n matter).
// free[d] points towards the latest free day ≤ d; day 0 means "none left". Taking a day
// links it to the day before (union-find with path halving).
$free = range(0, $n);
$total = 0;
foreach ($jobs as [$fee, $deadline]) {
    $d = min($deadline, $n);
    while ($free[$d] !== $d) {
        $free[$d] = $free[$free[$d]];
        $d = $free[$d];
    }
    if ($d > 0) {
        $free[$d] = $d - 1;
        $total += $fee;
    }
}
echo $total, "\n";
