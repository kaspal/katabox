<?php
function biggestNumber(array $cards): string {
    $texts = array_map('strval', $cards);
    usort($texts, fn ($a, $b) => strcmp($b . $a, $a . $b));
    $joined = implode('', $texts);
    return $joined[0] === '0' ? '0' : $joined;
}
