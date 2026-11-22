<?php
function fewestTokens(array $tokens, int $amount): int {
    $inf = count($tokens) + 1;
    $best = array_fill(0, $amount + 1, $inf);
    $best[0] = 0;
    foreach ($tokens as $t) {
        for ($a = $amount; $a >= $t; $a--) {
            if ($best[$a - $t] + 1 < $best[$a]) {
                $best[$a] = $best[$a - $t] + 1;
            }
        }
    }
    return $best[$amount] < $inf ? $best[$amount] : -1;
}
