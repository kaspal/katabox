<?php
$lines = explode("\n", stream_get_contents(STDIN));
[$r, $c, $k] = array_map('intval', preg_split('/\s+/', trim($lines[0])));
$out = [];
for ($y = 1; $y <= $r; $y++) {
    $row = trim($lines[$y]);
    $wide = '';
    for ($x = 0; $x < strlen($row); $x++) {
        $wide .= str_repeat($row[$x], $k);
    }
    for ($j = 0; $j < $k; $j++) {
        $out[] = $wide;
    }
}
echo implode("\n", $out), "\n";
