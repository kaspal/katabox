<?php
function pondSizes(array $ice): array {
    $h = count($ice);
    $w = strlen($ice[0]);
    $parent = range(0, $h * $w - 1);
    $size = array_fill(0, $h * $w, 1);
    $find = function (int $x) use (&$parent): int {
        while ($parent[$x] !== $x) {
            $parent[$x] = $parent[$parent[$x]];
            $x = $parent[$x];
        }
        return $x;
    };
    $union = function (int $a, int $b) use (&$parent, &$size, $find): void {
        $a = $find($a);
        $b = $find($b);
        if ($a === $b) {
            return;
        }
        if ($size[$a] < $size[$b]) {
            [$a, $b] = [$b, $a];
        }
        $parent[$b] = $a;
        $size[$a] += $size[$b];
    };
    for ($r = 0; $r < $h; $r++) {
        $row = $ice[$r];
        for ($c = 0; $c < $w; $c++) {
            if ($row[$c] !== '#') {
                continue;
            }
            if ($c + 1 < $w && $row[$c + 1] === '#') {
                $union($r * $w + $c, $r * $w + $c + 1);
            }
            if ($r + 1 < $h && $ice[$r + 1][$c] === '#') {
                $union($r * $w + $c, ($r + 1) * $w + $c);
            }
        }
    }
    $sizes = [];
    for ($r = 0; $r < $h; $r++) {
        for ($c = 0; $c < $w; $c++) {
            $i = $r * $w + $c;
            if ($ice[$r][$c] === '#' && $find($i) === $i) {
                $sizes[] = $size[$i];
            }
        }
    }
    rsort($sizes);
    return $sizes;
}
