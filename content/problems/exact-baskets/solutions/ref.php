<?php
function exactBaskets(array $weights, int $k, int $target): array {
    sort($weights);
    $out = [];
    $pick = [];
    $go = function (int $i, int $total) use (&$go, &$out, &$pick, $weights, $k, $target): void {
        if (count($pick) === $k) {
            if ($total === $target) {
                $out[] = $pick;
            }
            return;
        }
        for ($j = $i; $j < count($weights); $j++) {
            $pick[] = $weights[$j];
            $go($j + 1, $total + $weights[$j]);
            array_pop($pick);
        }
    };
    $go(0, 0);
    return $out;
}
