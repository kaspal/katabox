## Enfoque: fijar dos filas y aplicar Kadane sobre las columnas

Probar todos los rectángulos, incluso con sumas acumuladas, cuesta O(R²·C²): unos 1,6·10⁹ rectángulos en un campo de 10 × 8000. El truco es fijar solo las **filas** y dejar que un algoritmo de una dimensión elija las columnas.

Fija una fila superior `top` y una inferior `bottom`. Cada rectángulo con exactamente esas filas es un tramo de columnas consecutivas, y su total es la suma de ese tramo en el arreglo

```
col[j] = grid[top][j] + grid[top + 1][j] + ... + grid[bottom][j]
```

Así que el mejor rectángulo para ese par de filas es el **subarray no vacío de suma máxima** de `col`, que el **algoritmo de Kadane** encuentra en una pasada:

```
cur = 0
for x in col:
    cur = cur + x if cur > 0 else x   # mejor tramo que acaba aquí
    best = max(best, cur)
```

Un tramo que acaba en `x` o bien alarga el mejor tramo que acababa justo antes (solo compensa si ese tramo es positivo) o bien empieza de nuevo en `x`. Empezar `cur` en 0 hace que el primer paso elija el propio `x`, así que el tramo nunca está vacío, y un campo con todo negativo responde bien su mayor celda suelta.

Para cada `top`, empieza `col` a ceros y baja `bottom` desde `top`, sumando la fila `bottom` a `col` antes de cada pasada de Kadane. Cada rectángulo tiene una fila superior y una inferior, así que se considera exactamente una vez.

**Qué lado va fuera:** los pares de filas cuestan R²/2 pasadas de longitud C. Si R > C, traspón primero la cuadrícula (intercambia filas y columnas; la respuesta no cambia), así el coste es min(R, C)²·max(R, C)/2. Con R·C ≤ 8·10⁴ son como mucho unos 1,1·10⁷ pasos (un campo de 283 × 283), y solo 4·10⁵ para 10 × 8000.

**Complejidad:** tiempo O(min(R, C)²·max(R, C)), memoria O(R·C). Los totales no salen de ±8·10⁷, así que bastan enteros de 32 bits.

**Entrada:** lee todos los tokens de una vez; tras `R` y `C` vienen las `R·C` puntuaciones fila a fila. En Python, actualiza las sumas por columna con `list(map(operator.add, col, row))` (el bucle corre en C) y deja el bucle de Kadane lo más ajustado posible.
