<?php
function sharedMelody(array $first, array $second): int {
    $m = count($second);
    $prev = array_fill(0, $m + 1, 0);
    foreach ($first as $x) {
        $cur = array_fill(0, $m + 1, 0);
        for ($j = 1; $j <= $m; $j++) {
            if ($x === $second[$j - 1]) {
                $cur[$j] = $prev[$j - 1] + 1;
            } else {
                $cur[$j] = $prev[$j] > $cur[$j - 1] ? $prev[$j] : $cur[$j - 1];
            }
        }
        $prev = $cur;
    }
    return $prev[$m];
}
