<?php
function compress(string $text): string {
    $out = [];
    $n = strlen($text);
    $start = 0;
    for ($i = 1; $i <= $n; $i++) {
        if ($i === $n || $text[$i] !== $text[$start]) {
            $out[] = $text[$start];
            if ($i - $start > 1) {
                $out[] = (string) ($i - $start);
            }
            $start = $i;
        }
    }
    return implode('', $out);
}
