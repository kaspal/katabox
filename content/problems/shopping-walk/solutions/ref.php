<?php
function shortestWalk(string $stalls, string $need): int {
    $want = array_fill(0, 26, 0);
    $m = strlen($need);
    for ($k = 0; $k < $m; $k++) {
        $want[ord($need[$k]) - 97]++;
    }
    $have = array_fill(0, 26, 0);
    $missing = $m;
    $left = 0;
    $best = -1;
    $n = strlen($stalls);
    for ($right = 0; $right < $n; $right++) {
        $c = ord($stalls[$right]) - 97;
        if ($have[$c] < $want[$c]) {
            $missing--;
        }
        $have[$c]++;
        while ($missing === 0) {
            if ($best < 0 || $right - $left + 1 < $best) {
                $best = $right - $left + 1;
            }
            $d = ord($stalls[$left]) - 97;
            $have[$d]--;
            if ($have[$d] < $want[$d]) {
                $missing++;
            }
            $left++;
        }
    }
    return $best;
}
