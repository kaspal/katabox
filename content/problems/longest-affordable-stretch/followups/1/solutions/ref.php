<?php
function longestWithRefunds(array $costs, int $budget): int {
    $starts = [0];
    $where = [0];
    $prefix = 0;
    $best = 0;
    $n = count($costs);
    for ($j = 1; $j <= $n; $j++) {
        $prefix += $costs[$j - 1];
        $need = $prefix - $budget;
        $lo = 0;
        $hi = count($starts);
        while ($lo < $hi) {
            $m = intdiv($lo + $hi, 2);
            if ($starts[$m] < $need) {
                $lo = $m + 1;
            } else {
                $hi = $m;
            }
        }
        if ($lo < count($starts)) {
            $best = max($best, $j - $where[$lo]);
        }
        if ($prefix > $starts[count($starts) - 1]) {
            $starts[] = $prefix;
            $where[] = $j;
        }
    }
    return $best;
}
