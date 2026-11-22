<?php
function gridRoutes(array $floor): int {
    $mod = 1000000007;
    $rows = count($floor);
    $cols = strlen($floor[0]);
    // Bottom-up: ways[c] = routes from (r, c) to the bottom-right corner.
    $ways = array_fill(0, $cols + 1, 0);
    for ($r = $rows - 1; $r >= 0; $r--) {
        for ($c = $cols - 1; $c >= 0; $c--) {
            if ($floor[$r][$c] === '#') {
                $ways[$c] = 0;
            } elseif ($r === $rows - 1 && $c === $cols - 1) {
                $ways[$c] = 1;
            } else {
                $ways[$c] = ($ways[$c] + $ways[$c + 1]) % $mod;
            }
        }
    }
    return $ways[0];
}
