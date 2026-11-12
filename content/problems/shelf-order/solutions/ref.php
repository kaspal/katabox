<?php
function naturalOrder(array $labels): array {
    $keyOf = function (string $label): array {
        preg_match_all('/[0-9]+|[a-z]+/', $label, $m);
        $key = [];
        foreach ($m[0] as $chunk) {
            if (ctype_digit($chunk[0])) {
                $digits = ltrim($chunk, '0');
                $key[] = [0, strlen($digits), $digits];
            } else {
                $key[] = [1, 0, $chunk];
            }
        }
        return $key;
    };
    $keyed = [];
    foreach ($labels as $label) {
        $keyed[] = [$label, $keyOf($label)];
    }
    // usort is stable since PHP 8.0.
    usort($keyed, function ($x, $y) {
        $a = $x[1];
        $b = $y[1];
        $n = min(count($a), count($b));
        for ($i = 0; $i < $n; $i++) {
            if ($a[$i][0] !== $b[$i][0]) {
                return $a[$i][0] <=> $b[$i][0];
            }
            if ($a[$i][1] !== $b[$i][1]) {
                return $a[$i][1] <=> $b[$i][1];
            }
            $c = strcmp($a[$i][2], $b[$i][2]);
            if ($c !== 0) {
                return $c;
            }
        }
        return count($a) <=> count($b);
    });
    return array_map(fn ($x) => $x[0], $keyed);
}
