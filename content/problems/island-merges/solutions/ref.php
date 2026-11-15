<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$r = (int) $tokens[0];
$c = (int) $tokens[1];
$q = (int) $tokens[2];
$width = $c + 2; // a border of sea around the map: no bounds checks
$parent = array_fill(0, ($r + 2) * $width, -1); // -1: sea; otherwise union-find over land

function root(array &$parent, int $x): int
{
    while ($parent[$x] !== $x) {
        $parent[$x] = $parent[$parent[$x]];
        $x = $parent[$x];
    }
    return $x;
}

$islands = 0;
$out = [];
for ($k = 0; $k < $q; $k++) {
    $cell = (int) $tokens[3 + 2 * $k] * $width + (int) $tokens[4 + 2 * $k];
    if ($parent[$cell] === -1) {
        // A new island, then one fewer for every different island it touches.
        $parent[$cell] = $cell;
        $islands++;
        foreach ([$cell + 1, $cell - 1, $cell + $width, $cell - $width] as $next) {
            if ($parent[$next] !== -1) {
                $a = root($parent, $cell);
                $b = root($parent, $next);
                if ($a !== $b) {
                    $parent[$b] = $a;
                    $islands--;
                }
            }
        }
    }
    $out[] = $islands;
}
echo implode("\n", $out), "\n";
