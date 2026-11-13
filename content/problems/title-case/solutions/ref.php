<?php
function titleCase(string $text): string {
    $out = "";
    $atStart = true;
    $n = strlen($text);
    for ($i = 0; $i < $n; $i++) {
        $c = $text[$i];
        if ($c === " ") {
            $out .= $c;
            $atStart = true;
        } else {
            $out .= $atStart ? strtoupper($c) : strtolower($c);
            $atStart = false;
        }
    }
    return $out;
}
