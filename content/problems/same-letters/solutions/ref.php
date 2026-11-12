<?php
function sameLetters(string $a, string $b): bool {
    if (strlen($a) !== strlen($b)) {
        return false;
    }
    return count_chars($a, 1) === count_chars($b, 1);
}
