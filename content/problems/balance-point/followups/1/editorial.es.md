## Idea

El problema base comparaba dos lados; ahora tienen que coincidir tres tramos. Con sumas
prefijas `P`, los totales para los apoyos `i < j` son izquierda `P[i]`, centro
`P[j] − P[i] − weights[i]` y derecha `R = total − P[j + 1]`. Igualar izquierda y
derecha da `P[i] = R`, y entonces igualar centro y derecha da
`weights[i] = P[j] − 2R`. Así que para cada `j` necesitas cuántas posiciones anteriores
`i` tienen un par `(P[i], weights[i])` concreto. Recorre `j` de 1 a `n − 1`; antes de
revisar `j`, añade `i = j − 1` a un mapa hash que cuenta esos pares, y después suma a la
respuesta la cuenta de `(R, P[j] − 2R)`.

## Complejidad

O(n) en tiempo y O(n) de memoria extra. Las sumas prefijas quedan dentro de ±10⁹, pero
la respuesta puede llegar a unos 5·10⁹ (todo ceros): cuenta con un entero de 64 bits.
