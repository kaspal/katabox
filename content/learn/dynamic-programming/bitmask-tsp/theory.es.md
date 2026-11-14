## La idea

Un viajero sale de la ciudad 0, visita cada una de las demás ciudades exactamente una vez y vuelve a la ciudad 0. Conociendo la distancia entre cada par de ciudades, ¿cuál es el viaje de ida y vuelta más corto? Probar todos los órdenes de las otras `n - 1` ciudades supone `(n - 1)!` recorridos, que se disparan enseguida.

La observación clave (el método de **Held-Karp**): a mitad de viaje, el orden en que llegaste ya no importa, solo **qué ciudades** has visitado y **dónde estás ahora**. Así que `best[mask][last]` es el camino más corto que sale de la ciudad 0, visita exactamente las ciudades de `mask` y termina en `last`.

`mask` es una **máscara de bits**: un conjunto guardado en los bits de un número, donde el bit `c` vale 1 cuando la ciudad `c` está en el conjunto. Con cuatro ciudades, `mask = 11` es `1011` en binario: las ciudades 0, 1 y 3. Añadir la ciudad `c` es `mask | (1 << c)`, y `mask & (1 << c)` comprueba si `c` ya está.

Al principio solo se conoce `best[1][0] = 0`: solo la ciudad 0, estando en la ciudad 0. Todo lo demás empieza en un número enorme. El ejemplo recorre las máscaras en orden **creciente**, solo las impares (todas contienen la ciudad 0). Añadir una ciudad siempre hace el número mayor, así que cada estado es definitivo antes de ampliarlo. Desde cada `(mask, last)` alcanzado, prueba cada `city` no visitada y mejora `best[mask | (1 << city)][city]` con `best[mask][last] + dist[last][city]`. Al final, la máscara completa guarda el camino más corto por todas las ciudades que termina en cada `last`; sumar `dist[last][0]` cierra el ciclo, y el menor total es la salida.

Las distancias se escriben como una matriz simétrica, de hasta 5 ciudades.

### La entrada por defecto

Cuatro ciudades; la ciudad 0 está a 2, 9 y 10 de las ciudades 1, 2 y 3; de 1 a 2 hay 6, de 1 a 3 hay 4 y de 2 a 3 hay 3. Desde la máscara `0001`, los primeros movimientos dan `best[0011][1] = 2`, `best[0101][2] = 9` y `best[1001][3] = 10`. La máscara completa `1111` termina con 16 en la ciudad 1 (0, 2, 3, 1), 9 en la ciudad 2 (0, 1, 3, 2) y 11 en la ciudad 3 (0, 1, 2, 3). Cerrar el ciclo da `16 + 2 = 18`, `9 + 9 = 18` y `11 + 10 = 21`.

La salida es `[18]`: 0, 1, 3, 2, 0, o el mismo ciclo al revés.

## Pasos

1. Crea una tabla de `2ⁿ` máscaras por `n` ciudades, todas "sin alcanzar"; pon `best[1][0] = 0`.
2. Para cada máscara impar en orden creciente, cada `last` alcanzado y cada `city` que no esté en la máscara: mejora `best[mask | (1 << city)][city]` con `best[mask][last] + dist[last][city]`.
3. Devuelve el menor `best[completa][last] + dist[last][0]` con `last ≠ 0`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(2ⁿ · n²)`: cada máscara y última ciudad prueba todas las ciudades siguientes |
| Espacio extra | `O(2ⁿ · n)` para la tabla |

## Errores comunes

- **Olvidar la vuelta a casa.** La máscara completa guarda caminos abiertos; hay que sumar el `dist[last][0]` que los cierra.
- **Esperar que escale.** `2ⁿ · n²` es mucho mejor que `(n - 1)!` (con 15 ciudades, unos 7 millones de pasos frente a 87 000 millones de recorridos), pero sigue duplicándose con cada ciudad; el límite práctico ronda las 20.
- **Precedencia de bits.** En TypeScript y otros lenguajes de la familia de C, `mask & 1 << c === 0` compara antes de aplicar la máscara; añade paréntesis.

## Cuándo usarlo

Úsalo cuando hay que visitar todos los elementos de un conjunto pequeño en el mejor orden: rutas de reparto, ordenar tareas con costes de cambio, cubrir unos pocos puntos de control. El problema enlazado, Ruta de reparto, es este método sobre una matriz en la que las dos direcciones pueden ser distintas.

## Temas relacionados

- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): los desplazamientos y máscaras que se usan aquí.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): otra tabla sobre subconjuntos, un objeto cada vez.
- [Floyd-Warshall](learn:graphs/floyd-warshall): caminos más cortos entre todos los pares, cuando se permiten desvíos.
- [Caminos más cortos de Dijkstra](learn:graphs/dijkstra): caminos más cortos desde una ciudad, sin necesidad de visitarlas todas.
