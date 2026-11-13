<?php
$input = stream_get_contents(STDIN);
$newline = strpos($input, "\n");
$s = (int) substr($input, 0, $newline);
$lower = 'abcdefghijklmnopqrstuvwxyz';
$upper = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
// The encoded letter at position i decodes to the letter at position i - s.
$back = fn($alphabet) => substr($alphabet, 26 - $s) . substr($alphabet, 0, 26 - $s);
echo strtr(substr($input, $newline + 1), $lower . $upper, $back($lower) . $back($upper));
