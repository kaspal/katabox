<?php
function turnSignals(array $routes): array {
    $out = [];
    foreach ($routes as [$ax, $ay, $bx, $by, $cx, $cy]) {
        $cross = ($bx - $ax) * ($cy - $by) - ($by - $ay) * ($cx - $bx);
        $out[] = $cross > 0 ? "left" : ($cross < 0 ? "right" : "straight");
    }
    return $out;
}
