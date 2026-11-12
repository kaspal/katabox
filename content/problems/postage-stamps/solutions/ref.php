<?php
function exactPostage(array $values, int $target): array {
    $n = count($values);
    $suffix = array_fill(0, $n + 1, 0);
    for ($i = $n - 1; $i >= 0; $i--) {
        $suffix[$i] = $suffix[$i + 1] + $values[$i];
    }
    $best = null;
    $chosen = [];
    $search = function (int $i, int $missing) use (&$search, &$best, &$chosen, $values, $suffix, $n): void {
        if ($missing === 0) {
            if ($best === null || count($chosen) < count($best)) {
                $best = $chosen;
            }
            return;
        }
        if ($i === $n || $missing > $suffix[$i]) {
            return;
        }
        if ($best !== null && count($chosen) + 1 >= count($best)) {
            return;
        }
        if ($values[$i] <= $missing) {
            $chosen[] = $i;
            $search($i + 1, $missing - $values[$i]);
            array_pop($chosen);
        }
        $search($i + 1, $missing);
    };
    $search(0, $target);
    return $best ?? [];
}
