## Idea

Como en el problema base, convierte la temporada en un marcador acumulado (+1 por
victoria, −1 por derrota; los empates no lo cambian). La racha del partido `a + 1` al
partido `b` es equilibrada justo cuando el marcador tras `b` es igual al marcador tras
`a`. Para contar hacía falta cuántas veces salió cada marcador; para la más larga basta
la **primera** vez. Guarda `first[0] = −1` y, en cada partido `i`, o bien anota
`first[marcador] = i` (marcador nuevo) o bien prueba `i − first[marcador]` como mejor
longitud.

## Complejidad

O(n) en tiempo y O(n) de memoria extra (el marcador queda entre −n y n, así que un array
de tamaño 2n + 1 sirve igual que un mapa hash).
