<?php
function twoLonely(array $ids): array {
    $t = 0;
    foreach ($ids as $x) {
        $t ^= $x;
    }
    $bit = $t & -$t;
    $a = 0;
    foreach ($ids as $x) {
        if ($x & $bit) {
            $a ^= $x;
        }
    }
    $b = $t ^ $a;
    return $a < $b ? [$a, $b] : [$b, $a];
}
