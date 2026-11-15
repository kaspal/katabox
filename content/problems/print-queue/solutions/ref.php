<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$jobs = [];
for ($i = 0; $i < $n; $i++) {
    $jobs[] = [(int) $tokens[1 + 3 * $i], (int) $tokens[2 + 3 * $i], (int) $tokens[3 + 3 * $i], $i + 1];
}
usort($jobs, fn($x, $y) => [$x[0], $x[3]] <=> [$y[0], $y[3]]); // by arrival, then number

// SplPriorityQueue takes the largest priority first; [priority, -arrival, -number] puts
// the printer's next job on top (arrays compare element by element).
$waiting = new SplPriorityQueue();
$order = [];
$t = 0;
$next = 0;
while (count($order) < $n) {
    if ($waiting->isEmpty() && $jobs[$next][0] > $t) {
        $t = $jobs[$next][0]; // nothing to print: wait for the next job
    }
    while ($next < $n && $jobs[$next][0] <= $t) {
        [$a, $p, $g, $i] = $jobs[$next++];
        $waiting->insert([$i, $g], [$p, -$a, -$i]);
    }
    [$i, $g] = $waiting->extract();
    $order[] = $i;
    $t += $g;
}
echo implode(' ', $order), "\n";
