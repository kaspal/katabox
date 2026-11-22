<?php
function pondFences(array $ice): array {
    $rows = count($ice);
    $cols = strlen($ice[0]);
    $seen = array_fill(0, $rows * $cols, false);
    $fences = [];
    for ($r = 0; $r < $rows; $r++) {
        for ($c = 0; $c < $cols; $c++) {
            if ($ice[$r][$c] !== '#' || $seen[$r * $cols + $c]) {
                continue;
            }
            $seen[$r * $cols + $c] = true;
            $stack = [[$r, $c]];
            $edges = 0;
            while ($stack) {
                [$y, $x] = array_pop($stack);
                foreach ([[$y + 1, $x], [$y - 1, $x], [$y, $x + 1], [$y, $x - 1]] as [$ny, $nx]) {
                    if ($ny >= 0 && $ny < $rows && $nx >= 0 && $nx < $cols && $ice[$ny][$nx] === '#') {
                        if (!$seen[$ny * $cols + $nx]) {
                            $seen[$ny * $cols + $nx] = true;
                            $stack[] = [$ny, $nx];
                        }
                    } else {
                        $edges++;
                    }
                }
            }
            $fences[] = $edges;
        }
    }
    rsort($fences);
    return $fences;
}
