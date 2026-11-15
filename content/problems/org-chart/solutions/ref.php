<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$n = (int) $tokens[$pos++];
$team = array_fill(0, $n + 1, []);
for ($i = 2; $i <= $n; $i++) {
    $team[(int) $tokens[$pos++]][] = $i;
}
// Number the employees in depth-first order, with an explicit stack (a chart can be
// 10⁵ levels deep: far too deep for recursion). first[x] is x's number; last[x] the
// largest number in x's team, so the team is exactly first[x]..last[x].
$first = array_fill(0, $n + 1, 0);
$last = array_fill(0, $n + 1, 0);
$clock = 0;
$stack = [1]; // x to enter, or -x to leave
while ($stack) {
    $x = array_pop($stack);
    if ($x < 0) {
        $last[-$x] = $clock;
        continue;
    }
    $first[$x] = ++$clock;
    $stack[] = -$x;
    foreach ($team[$x] as $y) {
        $stack[] = $y;
    }
}
$q = (int) $tokens[$pos++];
$out = [];
for ($i = 0; $i < $q; $i++) {
    $a = (int) $tokens[$pos++];
    $b = (int) $tokens[$pos++];
    $out[] = $a !== $b && $first[$a] <= $first[$b] && $first[$b] <= $last[$a] ? 'YES' : 'NO';
}
echo implode("\n", $out), "\n";
