<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$n = (int) $tokens[0];
$k = (int) $tokens[1];
$ids = array_slice($tokens, 2, $n);

/** How many stretches have at most m different sources (a sliding window). */
function atMost(array $ids, int $m): int
{
    if ($m === 0) {
        return 0;
    }
    $count = [];
    $distinct = 0;
    $left = 0;
    $total = 0;
    foreach ($ids as $right => $x) {
        if (!isset($count[$x]) || $count[$x] === 0) {
            $count[$x] = 0;
            $distinct++;
        }
        $count[$x]++;
        while ($distinct > $m) {
            $y = $ids[$left++];
            if (--$count[$y] === 0) {
                $distinct--;
            }
        }
        // Every stretch ending at `right` and starting at left..right qualifies.
        $total += $right - $left + 1;
    }
    return $total;
}

// Exactly K = at most K minus at most K − 1.
echo atMost($ids, $k) - atMost($ids, $k - 1), "\n";
