<?php
$input = stream_get_contents(STDIN);
$newline = strpos($input, "\n");
$k = (int) substr($input, 0, $newline);
preg_match_all('/[a-z]+/', substr($input, $newline + 1), $matches);
// Words are letters only, so no key turns into an integer.
$counts = array_count_values($matches[0]);
$words = array_keys($counts);
usort($words, fn($a, $b) => ($counts[$b] <=> $counts[$a]) ?: strcmp($a, $b));
$out = [];
foreach (array_slice($words, 0, $k) as $word) {
    $out[] = $word . ' ' . $counts[$word];
}
echo implode("\n", $out), "\n";
