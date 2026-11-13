<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$present = [];
for ($i = 1; $i <= $n; $i++) {
    $present[(int) $tokens[$i]] = true;
}
// Distinct scores, highest first; then a map from score to rank.
$distinct = array_keys($present);
rsort($distinct);
$rank = [];
foreach ($distinct as $j => $score) {
    $rank[$score] = $j + 1;
}
$q = (int) $tokens[1 + $n];
$answers = [];
for ($j = 0; $j < $q; $j++) {
    $answers[] = $rank[(int) $tokens[2 + $n + $j]] ?? 0;
}
echo implode(" ", $answers), "\n";
