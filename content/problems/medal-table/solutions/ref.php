<?php
function medalTable(array $countries, array $medals): array {
    $order = array_keys($countries);
    usort($order, function ($a, $b) use ($countries, $medals) {
        for ($k = 0; $k < 3; $k++) {
            if ($medals[$a][$k] !== $medals[$b][$k]) {
                return $medals[$b][$k] <=> $medals[$a][$k];
            }
        }
        return strcmp($countries[$a], $countries[$b]);
    });
    return array_map(fn ($i) => $countries[$i], $order);
}
