<?php
function kthInWindow(array $scores, int $w, int $k): array {
    $count = array_fill(0, 301, 0);
    $out = [];
    $n = count($scores);
    for ($i = 0; $i < $n; $i++) {
        $count[$scores[$i]]++;
        if ($i >= $w) {
            $count[$scores[$i - $w]]--;
        }
        if ($i >= $w - 1) {
            $left = $k;
            $v = 300;
            while ($left > $count[$v]) {
                $left -= $count[$v];
                $v--;
            }
            $out[] = $v;
        }
    }
    return $out;
}
