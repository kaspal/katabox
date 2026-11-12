## Enfoque: profundidades y salto binario

El jefe común más cercano es el **ancestro común más bajo** (LCA) de `a` y `b` en el árbol que da `boss`.

**Preparación.** Construye las listas de hijos y recorre en anchura desde la presidenta para obtener la `profundidad` de cada socio (la presidenta tiene 0). Después construye la tabla de saltos:

```
up[0][v] = boss[v]   (la presidenta: ella misma)
up[k][v] = up[k − 1][ up[k − 1][v] ]    # el jefe 2^k niveles por encima de v
```

con `k` hasta ⌈log₂ n⌉. Cada nivel de la tabla se calcula a partir del anterior en O(n).

**Una consulta.**

1. Sea `a` el socio más profundo. Súbelo `d = profundidad[a] − profundidad[b]` niveles: para cada bit `k` activo en `d`, `a = up[k][a]`. Ahora los dos están a la misma profundidad.
2. Si `a == b`, ese socio es la respuesta (uno estaba por encima del otro).
3. Si no, para `k` del nivel más alto a 0: si `up[k][a] != up[k][b]`, haz `a = up[k][a]`, `b = up[k][b]`. Estos saltos nunca pasan del ancestro común, porque solo ocurren mientras los ancestros todavía difieren. Al final `a` y `b` son los dos hijos distintos justo por debajo de la respuesta.
4. La respuesta es `up[0][a]`.

### Paso a paso

Ejemplo 1, consulta (3, 5): profundidades 2 y 2. `up[1][3] = 0 = up[1][5]`, así que no se salta en el nivel 1; `up[0][3] = 1 ≠ 2 = up[0][5]`, así que se salta a a = 1, b = 2. Respuesta `up[0][1] = 0`.

**Complejidad:** O(n log n) para construir la tabla y O(log n) por consulta, con O(n log n) de memoria. Subir desde los dos socios en cada consulta cuesta O(q · n) en una jerarquía profunda.
