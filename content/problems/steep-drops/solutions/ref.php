<?php
function steepDrops(array $prices): int {
    $sort = function (array $a) use (&$sort, &$count): array {
        $n = count($a);
        if ($n < 2) {
            return $a;
        }
        $mid = intdiv($n, 2);
        $left = $sort(array_slice($a, 0, $mid));
        $right = $sort(array_slice($a, $mid));
        $m = count($right);
        $j = 0;
        foreach ($left as $x) {
            while ($j < $m && $x > 2 * $right[$j]) {
                $j++;
            }
            $count += $j;
        }
        $out = [];
        $i = 0;
        $k = 0;
        while ($i < $mid && $k < $m) {
            $out[] = $left[$i] <= $right[$k] ? $left[$i++] : $right[$k++];
        }
        while ($i < $mid) {
            $out[] = $left[$i++];
        }
        while ($k < $m) {
            $out[] = $right[$k++];
        }
        return $out;
    };
    $count = 0;
    $sort($prices);
    return $count;
}
