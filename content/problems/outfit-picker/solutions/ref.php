<?php
function outfits(array $prices, int $budget): int {
    $groups = $prices;
    foreach ($groups as &$g) {
        sort($g);
    }
    unset($g);
    $k = count($groups);
    $cheapest = array_fill(0, $k + 1, 0);
    for ($c = $k - 1; $c >= 0; $c--) {
        $cheapest[$c] = $cheapest[$c + 1] + $groups[$c][0];
    }
    $choose = function (int $c, int $spent) use (&$choose, $groups, $cheapest, $k, $budget): int {
        if ($c === $k) {
            return 1;
        }
        $total = 0;
        foreach ($groups[$c] as $price) {
            if ($spent + $price + $cheapest[$c + 1] > $budget) {
                break;
            }
            $total += $choose($c + 1, $spent + $price);
        }
        return $total;
    };
    return $choose(0, 0);
}
