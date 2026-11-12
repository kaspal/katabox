<?php
function largestTile(int $width, int $height): int {
    while ($height !== 0) {
        [$width, $height] = [$height, $width % $height];
    }
    return $width;
}
