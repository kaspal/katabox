<?php
function fenceArea(array $trees): int {
    $pts = $trees;
    usort($pts, fn ($a, $b) => $a[0] <=> $b[0] ?: $a[1] <=> $b[1]);
    $cross = fn ($o, $a, $b) => ($a[0] - $o[0]) * ($b[1] - $o[1]) - ($a[1] - $o[1]) * ($b[0] - $o[0]);
    $half = function (array $points) use ($cross): array {
        $hull = [];
        foreach ($points as $p) {
            while (count($hull) >= 2 && $cross($hull[count($hull) - 2], $hull[count($hull) - 1], $p) <= 0) {
                array_pop($hull);
            }
            $hull[] = $p;
        }
        return $hull;
    };
    $lower = $half($pts);
    $upper = $half(array_reverse($pts));
    $hull = array_merge(array_slice($lower, 0, -1), array_slice($upper, 0, -1));
    $h = count($hull);
    if ($h < 3) {
        return 0;
    }
    $area2 = 0;
    for ($i = 0; $i < $h; $i++) {
        [$x1, $y1] = $hull[$i];
        [$x2, $y2] = $hull[($i + 1) % $h];
        $area2 += $x1 * $y2 - $x2 * $y1;
    }
    return $area2;
}
