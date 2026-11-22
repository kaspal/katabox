La tabla del problema base tiene una celda por capacidad. Con `capacity` hasta 10⁹ esa
tabla es demasiado grande. ¿Qué magnitud es pequeña en este problema?
---
El valor total de todos los objetos es como mucho 100 · 100 = 10⁴. Indexa la tabla por
valor en lugar de por peso.
---
Sea `lightest[v]` el menor peso total de un grupo de objetos que vale exactamente `v`
(`lightest[0] = 0`, el resto inalcanzable). Añade los objetos uno a uno, recorriendo los
valores de mayor a menor como en el problema base. La respuesta es el mayor `v` con
`lightest[v] ≤ capacity`.
