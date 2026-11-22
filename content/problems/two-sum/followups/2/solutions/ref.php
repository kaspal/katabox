<?php
function closestSum(array $nums, int $target): int {
    $a = $nums;
    sort($a);
    $i = 0;
    $j = count($a) - 1;
    $best = $a[0] + $a[1];
    while ($i < $j) {
        $s = $a[$i] + $a[$j];
        $d = abs($s - $target);
        $bd = abs($best - $target);
        if ($d < $bd || ($d === $bd && $s < $best)) {
            $best = $s;
        }
        if ($s < $target) {
            $i++;
        } elseif ($s > $target) {
            $j--;
        } else {
            return $s;
        }
    }
    return $best;
}
