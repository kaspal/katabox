<?php
function twoSum(array $nums, int $target): array {
    $seen = [];
    foreach ($nums as $i => $n) {
        $need = $target - $n;
        if (array_key_exists($need, $seen)) {
            return [$seen[$need], $i];
        }
        $seen[$n] = $i;
    }
    return [];
}
