<?php
function swapsToSort(array $heights): int {
    // Bottom-up merge sort: merge runs of width 1, 2, 4, … between two buffers.
    $n = count($heights);
    $a = $heights;
    $count = 0;
    for ($width = 1; $width < $n; $width *= 2) {
        $b = [];
        for ($lo = 0; $lo < $n; $lo += 2 * $width) {
            $mid = min($lo + $width, $n);
            $hi = min($lo + 2 * $width, $n);
            $i = $lo;
            $j = $mid;
            while ($i < $mid && $j < $hi) {
                if ($a[$i] <= $a[$j]) {
                    $b[] = $a[$i++];
                } else {
                    $b[] = $a[$j++];
                    $count += $mid - $i;
                }
            }
            while ($i < $mid) {
                $b[] = $a[$i++];
            }
            while ($j < $hi) {
                $b[] = $a[$j++];
            }
        }
        $a = $b;
    }
    return $count;
}
