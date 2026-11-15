## La idea

Una consulta normal no puede seguir una cadena de longitud desconocida: "¿quién depende de quién, a cualquier profundidad?" necesita una unión por nivel, y nadie sabe de antemano cuántos niveles hay. Una **expresión de tabla común recursiva** (CTE recursiva) lo resuelve dejando que una consulta lea su propio resultado:

```sql
WITH RECURSIVE chain(id, depth) AS (
  SELECT id, 0 FROM emp WHERE boss IS NULL                     -- el ancla
  UNION ALL
  SELECT e.id, c.depth + 1 FROM emp e JOIN chain c ON e.boss = c.id  -- la parte recursiva
)
SELECT depth FROM chain ORDER BY id
```

Se ejecuta por rondas, no como una recursión de verdad:

- El **ancla** se ejecuta una vez. Aquí encuentra a los empleados sin jefe y les da profundidad 0.
- Cada **ronda** ejecuta la parte recursiva, pero `chain` representa en ella solo las filas encontradas en la ronda **anterior**, la **tabla de trabajo**. Unirlas con `emp` encuentra a los empleados que dependen de ellas, un nivel más abajo.
- Las filas nuevas se añaden al resultado y pasan a ser la siguiente tabla de trabajo. Cuando una ronda no encuentra nada, la consulta se detiene.

Así, las filas llegan nivel a nivel, exactamente como en una búsqueda en anchura desde la cima del árbol.

### La entrada por defecto

El arreglo `[-1, 0, 0, 1, 1, 2]` da el jefe de cada empleado: el empleado `i` depende de `a[i]`, y `-1` significa sin jefe. El empleado 0 es la cima; 1 y 2 dependen de 0; 3 y 4 dependen de 1; 5 depende de 2.

Ancla: el empleado 0 no tiene jefe, profundidad 0. La ronda 1 une la tabla de trabajo `{0}` y encuentra a 1 y 2, profundidad 1. La ronda 2 une `{1, 2}` y encuentra a 3 y 4 (jefe 1) y a 5 (jefe 2), profundidad 2. La ronda 3 une `{3, 4, 5}` y no encuentra a nadie, así que la consulta se detiene tras 3 rondas.

La salida es `[0, 1, 1, 2, 2, 2]`: la profundidad de cada empleado, por id.

## Pasos

1. Ancla: da profundidad 0 a cada empleado cuyo jefe es `-1`; forman la primera tabla de trabajo.
2. Ronda: para cada empleado `b` de la tabla de trabajo, encuentra cada empleado `e` con jefe `b` y dale a `e` profundidad `depth[b] + 1`.
3. Los empleados encontrados pasan a ser la siguiente tabla de trabajo.
4. Repite hasta que una ronda no encuentre a nadie y devuelve las profundidades por id.

## Coste

Con `n` empleados:

| Caso | Coste |
|---|---|
| Mejor | `O(n)` con un índice sobre `boss`: cada empleado se busca una vez |
| Promedio | `O(n · rondas)`, recorriendo la tabla una vez por ronda |
| Peor | `O(n²)`: una sola cadena larga necesita `n` rondas |
| Espacio extra | `O(n)` para el resultado y la tabla de trabajo |

El ejemplo lo mantiene sencillo y recorre todos los empleados por cada fila de la tabla de trabajo, la versión cuadrática.

## Errores comunes

- **Ciclos.** Si 1 depende de 2 y 2 depende de 1, cada ronda vuelve a encontrar una fila y `UNION ALL` no termina nunca. Protecciones: `UNION` en lugar de `UNION ALL` descarta las filas que ya están en el resultado (solo sirve cuando las filas repetidas son idénticas, así que no con una `depth` que crece), o una condición como `WHERE c.depth < 100` en la parte recursiva. La entrada de aquí es un árbol y nunca da vueltas.
- **Unir todo el resultado en cada ronda.** La parte recursiva solo ve la ronda anterior; creer que lo ve todo lleva a profundidades erróneas y filas duplicadas.
- **Olvidar el filtro del ancla.** Sin `WHERE boss IS NULL`, todos los empleados empiezan con profundidad 0.

## Cuándo usarlo

Usa una consulta recursiva para jerarquías y cadenas guardadas como columnas de "padre": organigramas, árboles de carpetas, listas de materiales, árboles genealógicos, o todos los nodos alcanzables en un grafo. Los problemas enlazados, Organigrama y Generaciones de la familia, recorren justo este tipo de lista de jefes o de padres.

## Temas relacionados

- Búsqueda en anchura (BFS): la misma frontera nivel a nivel.
- Búsqueda en profundidad (DFS): otra forma de recorrer un árbol desde su raíz.
- Recursión y pila de llamadas: la recursión de verdad, comparada con estas rondas.
- Algoritmos de unión: la unión que hace cada ronda.
