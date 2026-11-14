<?php
const MOD = 1000003;

function ways(string $msg): int
{
    // $msg[0] is the '#'; the digits start at index 1.
    // $before = ways of the prefix one digit shorter, $last = ways of the current prefix.
    $before = 1;
    $last = $msg[1] === '0' ? 0 : 1;
    $len = strlen($msg);
    for ($i = 2; $i < $len; $i++) {
        $c = $msg[$i];
        $p = $msg[$i - 1];
        $cur = $c !== '0' ? $last : 0;
        if ($p === '1' || ($p === '2' && $c <= '6')) { // "1x" or "20".."26"
            $cur += $before;
        }
        $before = $last;
        $last = $cur % MOD;
    }
    return $last;
}

$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$t = (int) $tokens[0];
$out = [];
for ($j = 1; $j <= $t; $j++) {
    $out[] = ways($tokens[$j]);
}
echo implode("\n", $out), "\n";
