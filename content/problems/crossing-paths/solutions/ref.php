<?php
function pathsCross(array $pairs): array {
    $orient = fn ($ax, $ay, $bx, $by, $cx, $cy) => ($bx - $ax) * ($cy - $ay) - ($by - $ay) * ($cx - $ax) <=> 0;
    $within = fn ($ax, $ay, $bx, $by, $px, $py) =>
        min($ax, $bx) <= $px && $px <= max($ax, $bx) && min($ay, $by) <= $py && $py <= max($ay, $by);
    $out = [];
    foreach ($pairs as [$x1, $y1, $x2, $y2, $x3, $y3, $x4, $y4]) {
        $d1 = $orient($x1, $y1, $x2, $y2, $x3, $y3);
        $d2 = $orient($x1, $y1, $x2, $y2, $x4, $y4);
        $d3 = $orient($x3, $y3, $x4, $y4, $x1, $y1);
        $d4 = $orient($x3, $y3, $x4, $y4, $x2, $y2);
        $out[] = ($d1 * $d2 < 0 && $d3 * $d4 < 0)
            || ($d1 === 0 && $within($x1, $y1, $x2, $y2, $x3, $y3))
            || ($d2 === 0 && $within($x1, $y1, $x2, $y2, $x4, $y4))
            || ($d3 === 0 && $within($x3, $y3, $x4, $y4, $x1, $y1))
            || ($d4 === 0 && $within($x3, $y3, $x4, $y4, $x2, $y2));
    }
    return $out;
}
