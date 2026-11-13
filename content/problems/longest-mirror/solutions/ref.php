<?php
function longestMirror(string $text): string {
    $n = strlen($text);
    $bestStart = 0;
    $bestLen = 0;
    // Middles in order: letter c (odd) then the gap after it (even).
    for ($c = 0; $c < $n; $c++) {
        foreach ([[$c, $c], [$c, $c + 1]] as [$lo, $hi]) {
            while ($lo >= 0 && $hi < $n && $text[$lo] === $text[$hi]) {
                $lo--;
                $hi++;
            }
            $len = $hi - $lo - 1;
            if ($len > $bestLen) {
                $bestLen = $len;
                $bestStart = $lo + 1;
            }
        }
    }
    return substr($text, $bestStart, $bestLen);
}
