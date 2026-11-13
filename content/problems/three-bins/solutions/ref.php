<?php
function sortBins(array $items): array {
    $low = 0;
    $mid = 0;
    $high = count($items) - 1;
    while ($mid <= $high) {
        if ($items[$mid] === 0) {
            [$items[$low], $items[$mid]] = [$items[$mid], $items[$low]];
            $low++;
            $mid++;
        } elseif ($items[$mid] === 1) {
            $mid++;
        } else {
            [$items[$mid], $items[$high]] = [$items[$high], $items[$mid]];
            $high--;
        }
    }
    return $items;
}
