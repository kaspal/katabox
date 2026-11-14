## La idea

Un **primo** es un número entero mayor que 1 cuyos únicos divisores son 1 y él mismo. Para enumerar todos los primos hasta `n`, podrías probar cada número dividiéndolo. La criba de Eratóstenes le da la vuelta a la pregunta: en lugar de preguntar "¿algo divide a `k`?", cada primo **tacha sus propios múltiplos**, y lo que nunca se tacha es primo.

Escribe los números de 0 a `n` (la aplicación los muestra de diez en diez por fila) y tacha el 0 y el 1. Toma `p = 2`: no está tachado, así que es primo, y sus múltiplos no lo son. Pasa a 3, 4, 5, … y repite con cada `p` que siga sin tachar cuando le llegue el turno. Un número que llega a su turno sin tachar no lo ha tachado ningún primo menor, así que nada menor lo divide: es primo.

Dos atajos lo hacen rápido:

- **Empezar en `p²`.** Un múltiplo menor `j · p` con `j < p` tiene el factor menor `j`, así que los factores primos de `j` ya lo tacharon.
- **Parar cuando `p² > n`.** Un número compuesto `k ≤ n` tiene un factor no mayor que `√k`, así que ya cayó con el primo de ese factor. Todo número que sigue en pie después de eso es primo.

La salida es el número de primos y después los primos en orden. La entrada es un único `n` de 2 a 120.

### La entrada por defecto

`n = 60`. Los primos 2, 3, 5 y 7 hacen el tachado; en `p = 8`, `8² = 64 > 60`, así que el bucle se detiene. El 2 tacha 4, 6, …, 60 (29 tachados), el 3 empieza en 9 (18 tachados), el 5 empieza en 25 (8 tachados) y el 7 solo tacha 49 y 56. Son 57 tachados, pero solo 42 números distintos: el 12, por ejemplo, lo alcanzan el 2 y el 3. El 49 es el único número que queda para el 7 solo.

La salida es `[17, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59]`: diecisiete primos hasta 60.

## Pasos

1. Marca cada número de 2 a `n` como posible primo; el 0 y el 1 no lo son.
2. Para `p = 2, 3, …` mientras `p · p ≤ n`: si `p` sigue marcado, tacha `p²`, `p² + p`, `p² + 2p`, … hasta `n`.
3. Reúne los números que siguen marcados; devuelve cuántos hay y después los números.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log log n)`: los tachados de todos los primos juntos |
| Espacio extra | `O(n)`: una marca por número |

El primo `p` tacha como mucho `n / p` números. Sumar `n / 2 + n / 3 + n / 5 + …` sobre los primos hasta `√n` crece como `n log log n`, que es casi lineal: `log log n` es menor que 4 para cualquier `n` que quepa en un ordenador.

## Errores comunes

- **Empezar en `2p`.** Sigue funcionando, pero cada tachado por debajo de `p²` es trabajo perdido.
- **Comparar con una raíz cuadrada de coma flotante.** El redondeo puede saltarse el último primo que debería tachar; compara `p · p ≤ n` con enteros.
- **Olvidar el 0 y el 1.** Ninguno es primo; si se quedan marcados, la cuenta sale con dos de más.
- **Un `n` enorme.** Una marca por número limita la criba a lo que cabe en memoria; para un solo número basta dividir hasta `√k`.

## Cuándo usarlo

Úsalo cuando necesites muchos primos, o muchas respuestas a "¿es primo?", hasta un límite fijo: criba una vez y cada respuesta es una consulta directa. El problema enlazado, Taquillas primas, pregunta cuántos primos hay hasta cada uno de muchos límites: una criba y una cuenta acumulada los responden todos.

## Temas relacionados

- [MCD de Euclides](learn:math-geometry/euclid-gcd): divisores comunes de dos números sin factorizarlos.
- [Sumas prefijas](learn:foundations/prefix-sums): convierten las marcas de la criba en "cuántos primos hasta `L`" en `O(1)` por pregunta.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): lo cerca que se queda `n log log n` de `n`.
