<?php
function menuCombos(array $prices, int $budget): array {
    $n = count($prices);
    $out = [];
    for ($mask = 1; $mask < (1 << $n); $mask++) {
        $total = 0;
        $dishes = [];
        for ($i = 0; $i < $n; $i++) {
            if (($mask >> $i) & 1) {
                $total += $prices[$i];
                $dishes[] = $i;
            }
        }
        if ($total === $budget) {
            $out[] = $dishes;
        }
    }
    // PHP compares arrays by length first, so compare element by element.
    usort($out, function ($a, $b) {
        $m = min(count($a), count($b));
        for ($i = 0; $i < $m; $i++) {
            if ($a[$i] !== $b[$i]) {
                return $a[$i] <=> $b[$i];
            }
        }
        return count($a) <=> count($b);
    });
    return $out;
}
