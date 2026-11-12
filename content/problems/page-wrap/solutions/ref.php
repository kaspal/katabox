<?php
function wrapLines(array $words, int $width): array {
    $lines = [];
    $current = [$words[0]];
    $length = strlen($words[0]);
    $n = count($words);
    for ($i = 1; $i < $n; $i++) {
        $w = $words[$i];
        if ($length + 1 + strlen($w) <= $width) {
            $current[] = $w;
            $length += 1 + strlen($w);
        } else {
            $lines[] = implode(' ', $current);
            $current = [$w];
            $length = strlen($w);
        }
    }
    $lines[] = implode(' ', $current);
    return $lines;
}
