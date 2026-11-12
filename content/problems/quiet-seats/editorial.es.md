## Enfoque: revisar el bloque de 3 × 3 de cada butaca libre

Para cada butaca libre `(r, c)`, mira las celdas `(r + dr, c + dc)` con `dr, dc ∈ {-1, 0, 1}` que están dentro del plano. La butaca es tranquila cuando ninguna es `#` (la propia butaca está libre, así que incluirla no cambia nada).

```
cuenta = 0
para r, c en el plano con rows[r][c] == '.':
    si no hay '#' en las celdas del bloque de 3 × 3 alrededor de (r, c) dentro del plano:
        cuenta += 1
```

### Paso a paso

En `["#...", "....", "..#."]`, la butaca de la fila 0, columna 2 tiene como vecinas `.` `.` (fila 0) y `.` `.` `.` (fila 1): tranquila. La de la fila 1, columna 1 toca en diagonal el `#` de la fila 2, columna 2: no es tranquila.

**Complejidad:** O(filas × columnas) de tiempo, con como mucho 8 comprobaciones por butaca, y O(1) de espacio adicional.
