<?php
function whereIsPoint(array $fence, array $points): array {
    $n = count($fence);
    $out = [];
    foreach ($points as [$px, $py]) {
        $result = null;
        $inside = false;
        for ($i = 0; $i < $n; $i++) {
            [$ax, $ay] = $fence[$i];
            [$bx, $by] = $fence[($i + 1) % $n];
            if (($bx - $ax) * ($py - $ay) - ($by - $ay) * ($px - $ax) === 0
                && min($ax, $bx) <= $px && $px <= max($ax, $bx)
                && min($ay, $by) <= $py && $py <= max($ay, $by)) {
                $result = "boundary";
                break;
            }
            if (($ay > $py) !== ($by > $py)) {
                $t = ($py - $ay) * ($bx - $ax) - ($px - $ax) * ($by - $ay);
                if (($t > 0) === ($by > $ay)) {
                    $inside = !$inside;
                }
            }
        }
        $out[] = $result ?? ($inside ? "inside" : "outside");
    }
    return $out;
}
