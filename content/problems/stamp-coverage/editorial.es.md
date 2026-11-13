## Enfoque: un array de diferencias en 2-D

Usa una tabla `d` con una fila y una columna extra. Para cada sello, escribe cuatro marcas:

- `+1` en `(top, left)`, donde empieza el rectángulo;
- `−1` en `(top, right + 1)` y en `(bottom + 1, left)`, justo después de sus bordes derecho e inferior;
- `+1` en `(bottom + 1, right + 1)`, porque esa esquina se anuló dos veces.

Ahora haz una suma acumulada a lo largo de cada fila y luego otra hacia abajo en cada columna. Cada casilla acaba con la suma de todas las marcas que tiene arriba y a la izquierda, que es exactamente el número de sellos que la cubren: las cuatro marcas de un sello se anulan en todas partes fuera de su rectángulo.

Por último, recorre las `rows × cols` casillas buscando el máximo y cuántas casillas lo tienen.

### Paso a paso

El ejemplo 3 es una sola fila: marcas `+1` en 0, `−1` en 2, `+1` en 3, `−1` en 5. La suma acumulada es `1 1 0 1 1`: el máximo 1 aparece 4 veces.

**Complejidad:** O(sellos + rows · cols) en tiempo y O(rows · cols) de espacio. Entintar cada casilla de cada sello es O(sellos · rows · cols), unos 10¹⁰ pasos con 6·10⁴ sellos que cubren casi toda una hoja de 500 × 500.
