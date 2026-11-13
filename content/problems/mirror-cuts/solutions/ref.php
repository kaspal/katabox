<?php
function mirrorCuts(string $text): int {
    $n = strlen($text);
    $cuts = [];
    for ($i = 0; $i <= $n; $i++) {
        $cuts[] = $i - 1;
    }
    for ($c = 0; $c < $n; $c++) {
        foreach ([[$c, $c], [$c, $c + 1]] as [$l, $r]) {
            while ($l >= 0 && $r < $n && $text[$l] === $text[$r]) {
                if ($cuts[$l] + 1 < $cuts[$r + 1]) {
                    $cuts[$r + 1] = $cuts[$l] + 1;
                }
                $l--;
                $r++;
            }
        }
    }
    return $cuts[$n];
}
