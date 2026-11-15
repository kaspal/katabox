<?php
$s = trim(stream_get_contents(STDIN));
// The longest palindrome at the start of s is the longest prefix of s that is also a
// suffix of reversed(s). The prefix function of s . "#" . reversed(s) gives exactly that
// at its last position ("#" stops a match from running across the middle).
$reversed = strrev($s);
$t = $s . '#' . $reversed;
$len = strlen($t);
$pi = array_fill(0, $len, 0);
for ($i = 1; $i < $len; $i++) {
    $k = $pi[$i - 1];
    while ($k > 0 && $t[$i] !== $t[$k]) {
        $k = $pi[$k - 1];
    }
    if ($t[$i] === $t[$k]) {
        $k++;
    }
    $pi[$i] = $k;
}
$keep = $pi[$len - 1];
echo substr($reversed, 0, strlen($s) - $keep), $s, "\n";
