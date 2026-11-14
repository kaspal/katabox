## La idea

Tim sort, la ordenación que hay detrás de la ordenación de Python y de la de Java para objetos, combina dos virtudes: la ordenación por inserción es rápida con tramos cortos y la mezcla es rápida con tramos largos ya ordenados. Este tema muestra una versión **simplificada**. La ordenación por inserción convierte cada bloque de `RUN = 4` valores en un **tramo** ordenado; luego los tramos se mezclan por parejas, con un ancho que se duplica (4, 8, 16, …) hasta que un solo tramo cubre todo el arreglo. La mezcla es el mismo `merge` con búfer de la ordenación por mezcla.

### Lo que añade el Tim sort real

Este tema deja fuera todo esto:

- **Busca tramos naturales**: trozos que ya son ascendentes (los estrictamente descendentes los invierte) en lugar de cortar bloques fijos.
- **Elige la longitud mínima de tramo a partir de `n`** (normalmente entre 32 y 64) y alarga los tramos naturales cortos hasta ella con inserción.
- Mantiene una **pila de tramos** y los mezcla según reglas sobre sus longitudes, para que las mezclas queden equilibradas.
- **Galopa**: cuando un tramo gana una y otra vez durante una mezcla, salta hacia delante con una búsqueda binaria en lugar de comparar valor a valor.

## Pasos

1. Corta el arreglo en bloques de `RUN = 4` (el último puede ser más corto) y ordena cada uno por inserción.
2. Pon `width = RUN`.
3. Para cada pareja de tramos vecinos `lo..mid` y `mid + 1..hi`, mézclalos a través del búfer. Un tramo sin pareja espera.
4. Duplica `width` y repite mientras `width < n`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` en el Tim sort real con entrada ordenada; esta versión sigue mezclando, `O(n log n)` |
| Caso medio | `O(n log n)` |
| Peor caso | `O(n log n)` |
| Espacio extra | `O(n)` para el búfer |
| Estable | Sí: tanto la inserción como la mezcla mantienen el orden de los empates |
| En el sitio | No |

## Errores comunes

- **El tramo que sobra.** Con 10 valores los tramos son 0–3, 4–7 y 8–9. En la pasada de ancho 4, el 8–9 no tiene pareja y espera; la pasada de ancho 8 lo mezcla con 0–7. El límite del bucle `lo < n - width` es lo que lo salta.
- **Los bloques fijos desaprovechan el orden.** Aquí una entrada ordenada pasa igualmente por todas las mezclas. Buscar tramos naturales es lo que hace que el Tim sort real sea `O(n)` con ella.
- **Empates.** Si la inserción o la mezcla dejan que un valor adelante a otro igual, la ordenación deja de ser estable.

## Cuándo usarlo

Rara vez escribirás Tim sort tú mismo: es lo que ya hace la ordenación de la biblioteca de tu lenguaje. Conocerlo explica por qué ordenar datos casi ordenados es rápido en Python y por qué las ordenaciones de biblioteca son estables.

## Temas relacionados

- [Ordenación por inserción](learn:sorting/insertion-sort): construye los tramos.
- [Ordenación por mezcla](learn:sorting/merge-sort): el mismo `merge`.
- [Búsqueda binaria](learn:searching/binary-search): la que usa el galope.
