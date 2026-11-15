## Enfoque: números de entrada en profundidad y rangos de equipo

La respuesta obvia sube desde `b`, de jefe en jefe, hasta encontrar a `a` o pasar por el director. Eso cuesta la profundidad de `b` por pregunta. En un organigrama que es una sola línea de 10⁵ personas, con 2·10⁴ preguntas sobre la cabeza y el final de la línea, son unos 10⁹ pasos. En su lugar, numera el organigrama una vez para que cada pregunta se reduzca a dos comparaciones.

**Números de entrada.** Construye el equipo de cada empleado (la lista de personas de las que es jefe) y recorre el organigrama en profundidad desde el director. Da a cada empleado un número de entrada `first[x]` cuando la búsqueda lo alcance y, cuando la búsqueda haya terminado con todos los que están por debajo de `x`, guarda `last[x]`, el mayor número de entrada repartido hasta ese momento. Como la búsqueda entra en `x` y visita a todos los que están por debajo de `x` antes de salir, esas personas reciben exactamente los números `first[x] + 1 .. last[x]`.

En el Ejemplo 1 (jefes `1 1 2 2 4`), visitando primero al último miembro de cada equipo:

```
empleado   1  3  2  5  4  6
first      1  2  3  4  5  6
last       6  2  6  4  6  6
```

El rango de `2` es `3..6`, que contiene a `5`, `4` y `6`: exactamente los que están por debajo de `2`.

**Una pregunta.** `a` está por encima de `b` exactamente cuando `a ≠ b` y

```
first[a] ≤ first[b] ≤ last[a]
```

**Por qué es correcto:** si `b` está por debajo de `a`, la búsqueda alcanza a `b` mientras está dentro del equipo de `a`, así que `first[b]` cae en el rango de `a`. Si `b` no está por debajo de `a`, la búsqueda alcanza a `b` antes de entrar en `a` o después de haber salido para siempre del equipo de `a`, así que `first[b]` queda fuera del rango. La condición `a ≠ b` descarta el caso `first[b] = first[a]`.

**Sin recursión.** La búsqueda recursiva natural baja tanto como el organigrama, y un organigrama puede tener 10⁵ niveles: Python se detiene en su límite de recursión (1000 por defecto), y TypeScript, PHP y Rust pueden desbordar su pila de llamadas. Usa una pila explícita de pares `(empleado, terminado)`: al sacar `(x, false)` le das a `x` su número de entrada, apilas `(x, true)` y después el equipo de `x`; al sacar `(x, true)` fijas `last[x]`, porque para entonces ya se ha visitado a todos los que se apilaron después.

**No numeres por orden de empleado.** Un jefe puede tener un número mayor que el del empleado, así que no puedes rellenar profundidades ni rangos en una sola pasada de `2` a `n`; la búsqueda funciona con cualquier numeración.

**Complejidad:** O(n) para construir los equipos y hacer la búsqueda, O(1) por pregunta: tiempo O(n + Q) y memoria O(n). Todos los números caben de sobra en un entero de 32 bits.

**Entrada y salida rápidas:** lee todos los tokens de una vez (la segunda línea vacía cuando `n` es 1 simplemente no aporta tokens) y escribe todas las respuestas de una sola vez.
