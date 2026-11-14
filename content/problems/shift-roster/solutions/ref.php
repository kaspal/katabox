<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$names = [];
$hours = [];
$total = 0;
for ($i = 0; $i < $n; $i++) {
    $names[] = $tokens[1 + 2 * $i];
    $h = (int) $tokens[2 + 2 * $i];
    $hours[] = $h;
    $total += $h;
}
$out = [(string) $total];
for ($i = 0; $i < $n; $i++) {
    // hours > total / n, in whole numbers
    if ($hours[$i] * $n > $total) {
        $out[] = $names[$i] . ' ' . $hours[$i];
    }
}
if (count($out) === 1) {
    $out[] = 'none';
}
echo implode("\n", $out), "\n";
