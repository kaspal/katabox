<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$n = (int) $tokens[$pos++];
$ids = array_slice($tokens, $pos, $n);
$pos += $n;
$q = (int) $tokens[$pos++];
$byEnd = array_fill(0, $n + 1, []);
$starts = [];
for ($k = 0; $k < $q; $k++) {
    $starts[$k] = (int) $tokens[$pos++];
    $byEnd[(int) $tokens[$pos++]][] = $k;
}
// Offline, by right end: while the log is read up to photo r, mark only the latest
// photo of each species. Then the species of l..r are the marks in l..r. A Fenwick
// tree keeps the marks' prefix counts.
$tree = array_fill(0, $n + 1, 0);
$latest = [];
$answer = array_fill(0, $q, 0);
for ($r = 1; $r <= $n; $r++) {
    $id = $ids[$r - 1];
    if (isset($latest[$id])) {
        for ($i = $latest[$id]; $i <= $n; $i += $i & -$i) {
            $tree[$i]--;
        }
    }
    $latest[$id] = $r;
    for ($i = $r; $i <= $n; $i += $i & -$i) {
        $tree[$i]++;
    }
    foreach ($byEnd[$r] as $k) {
        $total = 0;
        for ($i = $r; $i > 0; $i -= $i & -$i) {
            $total += $tree[$i];
        }
        for ($i = $starts[$k] - 1; $i > 0; $i -= $i & -$i) {
            $total -= $tree[$i];
        }
        $answer[$k] = $total;
    }
}
echo implode("\n", $answer), "\n";
