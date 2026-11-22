Las cadenas sencillas como `[0, 1], [1, 2]` funcionan. Prueba con una persona que aparece primero en dos amistades, por ejemplo `[0, 1]` y luego `[0, 2]`, y comprueba quién sigue en el mismo club que 1.
---
`find` devuelve la raíz correcta del árbol tal como está. El problema está en cómo se unen dos clubes: ¿a qué elemento se le cambia el `parent`?
---
`parent[a] = b` sobrescribe el enlace propio de `a`, así que aquello a lo que `a` estaba unido (su club anterior) queda desconectado. Une las raíces: `ra, rb = find(a), find(b)` y, si son distintas, `parent[ra] = rb`. Acortar el camino dentro de `find` (`parent[x] = parent[parent[x]]`) es opcional; mantiene los árboles poco profundos.
