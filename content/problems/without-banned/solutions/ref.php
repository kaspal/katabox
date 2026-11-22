<?php
function withoutBanned(array $items, array $banned): array {
    $bad = array_flip($banned);
    return array_values(array_filter($items, fn($x) => !isset($bad[$x])));
}
