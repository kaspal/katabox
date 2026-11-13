<?php
function coinCombos(array $coins, int $target): array {
    sort($coins);
    $out = [];
    $bag = [];
    $pick = function (int $start, int $remaining) use (&$pick, &$out, &$bag, $coins): void {
        if ($remaining === 0) {
            $out[] = $bag;
            return;
        }
        for ($i = $start; $i < count($coins) && $coins[$i] <= $remaining; $i++) {
            $bag[] = $coins[$i];
            $pick($i, $remaining - $coins[$i]);
            array_pop($bag);
        }
    };
    $pick(0, $target);
    return $out;
}
