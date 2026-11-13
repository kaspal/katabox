<?php
function convertBase(string $number, int $source, int $target): string {
    $digits = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    $value = 0;
    $len = strlen($number);
    for ($i = 0; $i < $len; $i++) {
        $value = $value * $source + strpos($digits, $number[$i]);
    }
    if ($value === 0) {
        return '0';
    }
    $out = '';
    while ($value > 0) {
        $out = $digits[$value % $target] . $out;
        $value = intdiv($value, $target);
    }
    return $out;
}
