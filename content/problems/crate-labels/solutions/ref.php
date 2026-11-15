<?php
$lines = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $lines[0];
$total = [];
for ($i = 1; $i <= $n; $i++) {
    foreach (explode(';', $lines[$i]) as $entry) {
        [$name, $qty] = explode(':', $entry);
        $total[$name] = ($total[$name] ?? 0) + (int) $qty;
    }
}
ksort($total, SORT_STRING);
$out = [];
foreach ($total as $name => $sum) {
    $out[] = "$name $sum";
}
echo implode("\n", $out), "\n";
