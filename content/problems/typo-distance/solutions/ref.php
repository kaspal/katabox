<?php
function typoDistance(string $typed, string $meant): int {
    $n = strlen($typed);
    $m = strlen($meant);
    $prev = range(0, $m);
    for ($i = 1; $i <= $n; $i++) {
        $cur = [$i];
        $x = $typed[$i - 1];
        for ($j = 1; $j <= $m; $j++) {
            if ($x === $meant[$j - 1]) {
                $cur[$j] = $prev[$j - 1];
            } else {
                $cur[$j] = 1 + min($prev[$j], $cur[$j - 1], $prev[$j - 1]);
            }
        }
        $prev = $cur;
    }
    return $prev[$m];
}
