## Idea

Las cadenas de mando forman un árbol con la directora en la raíz. Recórrelo desde arriba, un nivel cada vez:

1. **Empezar** por la directora, con profundidad 0.
2. **Bajar un nivel**: cada persona cuyo jefe ya está en la lista entra en ella, un nivel más abajo que ese jefe. Repetir hasta que no entre nadie nuevo.
3. **Poner nombres y ordenar** las filas.

Una expresión de tabla común recursiva hace justo esto. Su primer `SELECT` da las filas de partida; el segundo `SELECT` lee las filas encontradas hasta el momento y produce las siguientes. SQLite vuelve a pasar cada fila nueva por la segunda parte hasta que no aparecen más.

## La consulta

```sql
WITH RECURSIVE chain(id, depth) AS (
  SELECT id, 0 FROM staff WHERE boss_id IS NULL
  UNION ALL
  SELECT s.id, chain.depth + 1
  FROM staff s
  JOIN chain ON s.boss_id = chain.id
)
SELECT s.name, chain.depth
FROM chain
JOIN staff s ON s.id = chain.id
ORDER BY chain.depth, s.name, s.id;
```

## Detalles que importan

- **Mirar un solo nivel no basta.** Una consulta que solo comprueba si `boss_id` es `NULL` da 0 a la directora y 1 a todos los demás. En el ejemplo 1 eso deja a Dev y a Ada con profundidad 1 en lugar de 2 y 3. Añadir un número fijo de autouniones tampoco lo arregla: una cadena puede tener 1000 personas, así que el número de niveles solo se conoce recorriéndolos.
- **A cada persona se llega una vez.** No hay ciclos y cada uno tiene un solo jefe, así que el recorrido llega a cada persona por un único camino y `UNION ALL` no produce duplicados. (Con un ciclo, la profundidad seguiría creciendo y la recursión no terminaría nunca; las restricciones lo descartan.)
- **Las cadenas largas no son un problema.** SQLite ejecuta un CTE recursivo con una cola, no con llamadas anidadas, así que una cadena de 1000 niveles no choca con ningún límite de profundidad.
- **Nombres repetidos.** Dos personas pueden llamarse igual, así que une y ordena por `id`, nunca solo por `name`. Ordenar por `id` al final deja el orden completo.
- **Sin clave foránea declarada.** El esquema no declara `boss_id` como referencia, pero la unión solo compara valores, y cada `boss_id` corresponde a una persona real.
- **Hacia abajo, no hacia arriba.** También se puede partir de cada persona y subir hasta la directora contando los pasos. Funciona, pero recorre la cadena otra vez por cada persona que hay en ella (hasta unas 500.000 filas para una sola cadena larga); bajando desde la directora se visita a cada persona una vez.

## Coste

Cada persona entra en `chain` una vez. Para cada una, SQLite busca en `staff` a quienes dependen directamente de ella; con un índice en `boss_id` (SQLite puede crear uno automático) eso suma unos `O(n log n)` pasos, e incluso recorrer la tabla entera por cada fila, unas `n²` = 10⁶ comparaciones para 1000 personas, es rápido. La unión final por la clave primaria y la ordenación añaden `O(n log n)`.
