## La idea

Sumar un tramo de un arreglo valor a valor cuesta un paso por valor, cada vez que lo preguntas. Las sumas prefijas pagan ese coste una sola vez. Construye una fila `p` con una celda más que el arreglo, donde `p[i]` es la suma de todo lo que queda a la izquierda de la posición `i` (así que `p[0] = 0`). Entonces la suma de las posiciones `lo..hi` es simplemente `p[hi + 1] − p[lo]`: el total hasta el final del tramo menos el total de antes. Dos lecturas, mida lo que mida el tramo.

## Pasos

1. Crea `p` con `len(a) + 1` celdas y pon `p[0] = 0`.
2. Para cada `i`, de izquierda a derecha: `p[i + 1] = p[i] + a[i]`.
3. Para sumar las posiciones `lo..hi`, lee `p[hi + 1]` y `p[lo]`.
4. Devuelve `p[hi + 1] − p[lo]`.

## Coste

| Medida | Coste |
|---|---|
| Construcción | `O(n)`: una suma por valor |
| Cada suma de un tramo | `O(1)`: dos lecturas y una resta |
| Cambiar un valor | `O(n)`: cambian todos los prefijos posteriores |
| Espacio extra | `O(n)`: `n + 1` celdas |

## Errores comunes

- **Errores de uno.** El tramo `lo..hi` incluye los dos extremos, así que es `p[hi + 1] − p[lo]`, no `p[hi] − p[lo]` (eso dejaría fuera `a[hi]`). La celda extra `p[0] = 0` es la que hace que un tramo que empieza en 0 funcione sin casos especiales.
- **Actualizar el arreglo.** Cuando cambia `a[i]`, todos los `p[j]` con `j > i` quedan desfasados. Si los valores cambian entre consultas, usa un árbol de Fenwick o un árbol de segmentos.
- **Desbordamiento.** Un prefijo puede ser mucho mayor que cualquier valor suelto. En lenguajes con enteros de tamaño fijo, suma en un tipo de 64 bits. Los números de JavaScript pierden precisión por encima de 2⁵³.

## Cuándo usarlo

Usa sumas prefijas siempre que un arreglo no cambie y necesites muchas sumas de tramos: totales entre dos fechas, cuántas coincidencias hay entre dos posiciones o buscar el punto donde las sumas de la izquierda y la derecha se equilibran. La misma idea sirve con recuentos, con XOR en lugar de suma y en dos dimensiones para sumar rectángulos.

## Temas relacionados

- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): `q` consultas cuestan `O(n + q)` en lugar de `O(n · q)`.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): los prefijos con XOR funcionan igual.
- Árbol de Fenwick (próximamente): sumas prefijas que además admiten cambios.
- Ventana deslizante (próximamente).
