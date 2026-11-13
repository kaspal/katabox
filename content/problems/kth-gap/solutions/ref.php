<?php
function kthGap(array $posts, int $k): int {
    sort($posts);
    $n = count($posts);
    $lo = 0;
    $hi = $posts[$n - 1] - $posts[0];
    while ($lo < $hi) {
        $mid = intdiv($lo + $hi, 2);
        $count = 0;
        $i = 0;
        for ($j = 0; $j < $n; $j++) {
            while ($posts[$j] - $posts[$i] > $mid) {
                $i++;
            }
            $count += $j - $i;
        }
        if ($count >= $k) {
            $hi = $mid;
        } else {
            $lo = $mid + 1;
        }
    }
    return $lo;
}
