## La idea

La **envolvente convexa** de un conjunto de puntos es el polígono convexo más pequeño que los contiene a todos: estira una goma elástica alrededor de los puntos y suéltala. Sus esquinas son algunos de los puntos; el resto queda dentro o sobre sus lados.

La herramienta es el **producto cruzado** de tres puntos `o`, `a`, `b`:

`cross(o, a, b) = (a.x - o.x) · (b.y - o.y) - (a.y - o.y) · (b.x - o.x)`

Su signo dice hacia dónde giras al ir de `o` a `a` y después seguir hasta `b`: **positivo** es un giro a la izquierda (antihorario), **negativo** un giro a la derecha (horario), y **cero** significa que los tres puntos están en una misma recta.

La **cadena monótona de Andrew** ordena los puntos por `x`, después por `y`, y elimina los duplicados. Construye la **cadena inferior** de izquierda a derecha: antes de añadir un punto, mientras los dos últimos puntos de la cadena y el nuevo no formen un giro estricto a la izquierda (`cross ≤ 0`), se descarta el último, porque queda sobre o por encima de la recta que se lo salta. Después construye la **cadena superior** de derecha a izquierda con la misma regla. Cada cadena termina donde empieza la otra, así que se quita el último punto de cada una antes de unirlas. La salida enumera las esquinas en sentido antihorario, empezando por el primer punto en el orden (el de más a la izquierda y, si empatan, el más bajo). Los puntos sobre un lado se descartan, así que solo quedan las esquinas de verdad. Los puntos se escriben como pares `x y`, con coordenadas de 0 a 20.

### La entrada por defecto

Nueve puntos (el ajuste "unos pocos dentro"). Ordenados: (2, 3), (4, 13), (6, 10), (8, 7), (9, 1), (11, 16), (12, 9), (15, 4), (17, 11).

Cadena inferior: (4, 13), (6, 10) y (8, 7) se descartan uno tras otro, porque cada uno forma un giro a la derecha con el punto siguiente. (9, 1) se queda. (11, 16) lo descarta (12, 9), al que descarta (15, 4). La cadena inferior es (2, 3), (9, 1), (15, 4), (17, 11).

Cadena superior, desde (17, 11): (15, 4) y (12, 9) se descartan; (11, 16) se queda; (9, 1), (8, 7) y (6, 10) se descartan uno tras otro, y (4, 13) y (2, 3) se quedan. La cadena superior es (17, 11), (11, 16), (4, 13), (2, 3).

La salida es `[2, 3, 9, 1, 15, 4, 17, 11, 11, 16, 4, 13]`: seis esquinas. (8, 7), (12, 9) y (6, 10) quedan dentro.

## Pasos

1. Ordena los puntos por `x`, después por `y`, y elimina los duplicados. Si hay dos o menos, devuélvelos.
2. Para cada punto de izquierda a derecha: mientras la cadena inferior tenga dos puntos y `cross(penúltimo, último, p) ≤ 0`, descarta el último; después añade `p`.
3. Haz lo mismo de derecha a izquierda para la cadena superior.
4. Devuelve la cadena inferior sin su último punto y después la cadena superior sin su último punto.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log n)`: la ordenación; las cadenas son `O(n)` |
| Espacio extra | `O(n)` para las cadenas |

Cada punto se añade a una cadena una vez y se descarta como mucho una vez, así que construir las dos cadenas es lineal.

## Errores comunes

- **`< 0` en lugar de `≤ 0`.** Los puntos sobre un lado se quedan entonces como esquinas falsas; el ajuste "un cuadrado" conserva así (10, 4) y (4, 10).
- **Duplicados.** Dos copias de un mismo punto dan un producto cruzado cero en sitios inesperados; elimínalas primero.
- **Desbordamiento.** El producto cruzado multiplica diferencias de coordenadas; con coordenadas grandes hacen falta enteros de 64 bits.

## Cuándo usarlo

Úsalo para envolver un conjunto de puntos: cercas, contornos de colisión, el par de puntos más alejados, o como primer paso de cualquier pregunta sobre el exterior de una forma. El problema enlazado, La cerca de los árboles, pide el área dentro de la envolvente de hasta 10⁵ árboles.

## Temas relacionados

- [Intersección de segmentos](learn:math-geometry/segment-intersection): el mismo producto cruzado, usado para saber si dos segmentos se tocan.
- [Pila monótona](learn:searching/monotonic-stack): cada cadena es una pila que desapila mientras falla una condición.
- [Ordenación por mezcla (Merge Sort)](learn:sorting/merge-sort): la ordenación que fija el coste `O(n log n)`.
