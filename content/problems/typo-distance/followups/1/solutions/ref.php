<?php
function withinFixes(string $typed, string $meant, int $k): bool {
    $n = strlen($typed);
    $m = strlen($meant);
    if (abs($n - $m) > $k) {
        return false;
    }
    $big = $k + 1;
    $w = 2 * $k + 1;
    $prev = array_fill(0, $w, $big);
    for ($j = 0; $j <= min($m, $k); $j++) {
        $prev[$j + $k] = $j;
    }
    for ($i = 1; $i <= $n; $i++) {
        $cur = array_fill(0, $w, $big);
        $lo = max(0, $i - $k);
        $hi = min($m, $i + $k);
        for ($j = $lo; $j <= $hi; $j++) {
            $d = $j - $i + $k;
            if ($j === 0) {
                $cur[$d] = min($i, $big);
                continue;
            }
            $best = $prev[$d] + ($typed[$i - 1] === $meant[$j - 1] ? 0 : 1);
            if ($d + 1 < $w) {
                $best = min($best, $prev[$d + 1] + 1);
            }
            if ($d - 1 >= 0) {
                $best = min($best, $cur[$d - 1] + 1);
            }
            $cur[$d] = min($best, $big);
        }
        $prev = $cur;
    }
    return $prev[$m - $n + $k] <= $k;
}
