## El error

`routes(r, c)` cuenta las rutas desde la celda `(r, c)` hasta el final, y ese número depende de la celda. El código inicial lo guarda con la clave `r + c`:

```python
        if r + c in memo:
            return memo[r + c]
        memo[r + c] = (routes(r + 1, c) + routes(r, c + 1)) % MOD
        return memo[r + c]
```

Todas las celdas de una misma antidiagonal (mismo `r + c`) comparten una entrada, así que la primera que se calcula responde por las demás. Las celdas de una misma diagonal suelen tener distinto número de rutas, aunque cuadrículas diminutas como las de los ejemplos pueden salir bien por suerte.

Con `floor = ["..", "..", ".."]` (3 filas y 2 columnas): la recursión baja primero hasta `(2, 0)`, que tiene 1 ruta y se guarda con la clave 2. Después `(1, 1)` encuentra la clave 2 y también obtiene 1 (correcto por suerte), así que `(1, 0)` vale 2 y se guarda con la clave 1. Luego `(0, 1)` encuentra la clave 1 y obtiene 2, pero en realidad tiene solo 1 ruta (bajar en línea recta). El código devuelve 2 + 2 = 4 en lugar de 3.

## La corrección

Usa la celda como clave de la memoria:

```python
        if (r, c) in memo:
            return memo[(r, c)]
        memo[(r, c)] = (routes(r + 1, c) + routes(r, c + 1)) % MOD
        return memo[(r, c)]
```

En TypeScript, usa una clave única por celda, como `r * cols + c`, tanto en `memo.get` como en `memo.set`.

**Complejidad:** O(rows · cols) en tiempo y espacio extra; cada celda se calcula una vez.
