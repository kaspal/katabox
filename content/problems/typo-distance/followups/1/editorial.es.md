## Idea

Usa la recurrencia del problema base, pero solo donde puede importar. La celda `(i, j)`
(la distancia entre las primeras `i` letras de `typed` y las primeras `j` de `meant`)
vale al menos `|i − j|`, porque hacen falta esas inserciones o borrados solo para igualar
las longitudes. Así que una celda con `|i − j| > k` ya es "más que `k`", y cualquier
camino que pase por ella termina por encima de `k`.

Si `|typed.length − meant.length| > k`, devuelve `false`. Si no, rellena cada fila solo
para `j` desde `i − k` hasta `i + k`, lee las celdas justo fuera de la banda como `k + 1`
y limita cada valor a `k + 1`. La tabla limitada sigue diciendo correctamente si cada
celda es como mucho `k`, así que la respuesta es si la última celda es como mucho `k`.

## Complejidad

O(typed.length · k) en tiempo, unas 2·10⁴ · 41 ≈ 8·10⁵ celdas, y O(k) de memoria extra si
solo guardas la banda de la fila anterior.
