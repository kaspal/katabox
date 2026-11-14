## La idea

La búsqueda binaria necesita los dos extremos del tramo antes de empezar. La búsqueda exponencial primero encuentra un tramo pequeño que tiene que contener el objetivo y luego hace la búsqueda binaria solo dentro de él. Comprueba `a[0]` y después prueba las posiciones 1, 2, 4, 8, … (duplicando un `bound` cada vez) mientras el valor que hay ahí siga siendo menor que el objetivo. En cuanto `a[bound]` deja de ser menor (o `bound` se pasa del final), el objetivo solo puede estar entre el límite anterior, `bound // 2`, y `bound`.

Como el límite se duplica, llegar a un objetivo en la posición `i` cuesta unas `log₂ i` pruebas, y el tramo que queda mide como mucho `i` posiciones. La búsqueda entera cuesta `O(log i)`, que depende de dónde está el objetivo, no de lo larga que sea la lista. Un objetivo cerca del principio se encuentra rápido incluso en una lista enorme.

## Pasos

1. Si `a[0] == target`, devuelve `0`.
2. Pon `bound = 1`.
3. Mientras `bound < len(a)` y `a[bound] < target`, duplícalo: `bound *= 2`.
4. El objetivo solo puede estar entre `lo = bound // 2` y `hi = min(bound, len(a) - 1)`.
5. Haz una búsqueda binaria en `lo..hi`: devuelve la posición si lo encuentras, `-1` si el tramo se queda vacío.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(1)` (el objetivo está en la posición 0) |
| Caso medio | `O(log i)`: `i` es la posición del objetivo |
| Peor caso | `O(log n)` (el objetivo está cerca del final o no está) |
| Espacio extra | `O(1)` |

En el peor caso hace más o menos el doble de comprobaciones que una búsqueda binaria normal: una ronda para encontrar el tramo y otra para buscar en él.

## Errores comunes

- **Salirse del final.** La duplicación puede pasarse de `len(a) - 1`. Comprueba `bound < len(a)` antes de leer `a[bound]`, y limita `hi` con `min`.
- **Empezar a duplicar desde 0.** `0 * 2` sigue siendo 0, así que el bucle no avanzaría nunca. Por eso `a[0]` se comprueba aparte y `bound` empieza en 1.
- **Buscar en un tramo demasiado ancho.** El límite anterior ya tenía un valor menor que el objetivo, así que el tramo empieza en `bound // 2`, no en 0. Empezar en 0 sigue funcionando, pero pierde el coste `O(log i)`.

## Cuándo usarlo

Úsala cuando el final de los datos no se conoce o está muy lejos: una secuencia ordenada que puedes leer por posición, un archivo ordenado enorme o una función que crece con su entrada y no tiene un límite superior claro. También compensa cuando los objetivos suelen estar cerca del principio. El modo galope de Tim Sort usa la misma duplicación para saltarse tramos largos al mezclar.

## Temas relacionados

- [Búsqueda binaria](learn:searching/binary-search): la segunda fase, sobre un tramo conocido.
- [Búsqueda lineal](learn:searching/linear-search): comprueba las posiciones de una en una en lugar de duplicar.
- [Tim Sort (simplificado)](learn:sorting/tim-sort): el galope duplica de la misma manera.
- [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array): otro sitio donde duplicar mantiene bajo un coste.
