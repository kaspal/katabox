<?php
function paintedLength(array $strokes): int {
    usort($strokes, fn($a, $b) => $a[0] <=> $b[0]);
    $total = 0;
    [$runStart, $runEnd] = $strokes[0];
    $n = count($strokes);
    for ($i = 1; $i < $n; $i++) {
        [$start, $end] = $strokes[$i];
        if ($start <= $runEnd) {
            $runEnd = max($runEnd, $end);
        } else {
            $total += $runEnd - $runStart;
            $runStart = $start;
            $runEnd = $end;
        }
    }
    return $total + $runEnd - $runStart;
}
