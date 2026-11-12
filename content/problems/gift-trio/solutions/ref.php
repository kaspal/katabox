<?php
function closestTrio(array $prices, int $target): int {
    sort($prices);
    $n = count($prices);
    $best = $prices[0] + $prices[1] + $prices[2];
    $bestGap = abs($best - $target);
    for ($i = 0; $i < $n - 2; $i++) {
        $a = $prices[$i];
        $lo = $i + 1;
        $hi = $n - 1;
        while ($lo < $hi) {
            $s = $a + $prices[$lo] + $prices[$hi];
            $gap = abs($s - $target);
            if ($gap < $bestGap || ($gap === $bestGap && $s < $best)) {
                $best = $s;
                $bestGap = $gap;
            }
            if ($s < $target) {
                $lo++;
            } elseif ($s > $target) {
                $hi--;
            } else {
                return $s;
            }
        }
    }
    return $best;
}
