<?php
function fewestShelves(array $widths, int $width): int {
    $books = $widths;
    rsort($books);
    $lower = intdiv(array_sum($books) + $width - 1, $width);
    $ffd = [];
    foreach ($books as $b) {
        $placed = false;
        foreach ($ffd as $k => $load) {
            if ($load + $b <= $width) {
                $ffd[$k] += $b;
                $placed = true;
                break;
            }
        }
        if (!$placed) {
            $ffd[] = $b;
        }
    }
    $best = count($ffd);
    $loads = [];
    $n = count($books);
    $place = function (int $i) use (&$place, &$best, &$loads, $books, $width, $lower, $n): void {
        if ($best === $lower) {
            return;
        }
        if ($i === $n) {
            $best = min($best, count($loads));
            return;
        }
        $b = $books[$i];
        $tried = [];
        $count = count($loads);
        for ($k = 0; $k < $count; $k++) {
            if ($loads[$k] + $b <= $width && !isset($tried[$loads[$k]])) {
                $tried[$loads[$k]] = true;
                $loads[$k] += $b;
                $place($i + 1);
                $loads[$k] -= $b;
            }
        }
        if ($count + 1 < $best) {
            $loads[] = $b;
            $place($i + 1);
            array_pop($loads);
        }
    };
    $place(0);
    return $best;
}
