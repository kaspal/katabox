<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$pos = 0;
$n = (int) $tokens[$pos++];
$m = (int) $tokens[$pos++];
$unlocks = array_fill(0, $n + 1, []);
$missing = array_fill(0, $n + 1, 0); // prerequisites not passed yet (a repeated rule counts twice)
for ($i = 0; $i < $m; $i++) {
    $a = (int) $tokens[$pos++];
    $b = (int) $tokens[$pos++];
    $unlocks[$a][] = $b;
    $missing[$b]++;
}
// Kahn's algorithm, one term at a time: a term is every course whose last missing
// prerequisite was passed in the term before.
$term = [];
for ($c = 1; $c <= $n; $c++) {
    if ($missing[$c] === 0) {
        $term[] = $c;
    }
}
$lines = [];
$passed = 0;
while ($term) {
    sort($term);
    $lines[] = implode(' ', $term);
    $passed += count($term);
    $following = [];
    foreach ($term as $a) {
        foreach ($unlocks[$a] as $b) {
            if (--$missing[$b] === 0) {
                $following[] = $b;
            }
        }
    }
    $term = $following;
}
if ($passed < $n) {
    echo "IMPOSSIBLE\n";
} else {
    echo count($lines), "\n", implode("\n", $lines), "\n";
}
