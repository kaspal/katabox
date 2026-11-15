## Idea

Una pareja de alumnos que comparten curso son dos filas de `enrollments` con el mismo `course`, así que la consulta es una **autounión** (self-join):

1. **Emparejar** las matrículas del mismo curso, quedándose solo con `e1.student_id < e2.student_id`.
2. **Reducir** el resultado a parejas de ids distintas.
3. **Poner nombre** a los dos alumnos y ordenar por sus ids.

## La consulta

```sql
SELECT sa.name AS student_a, sb.name AS student_b
FROM (
  SELECT DISTINCT e1.student_id AS a, e2.student_id AS b
  FROM enrollments e1
  JOIN enrollments e2 ON e2.course = e1.course AND e1.student_id < e2.student_id
) p
JOIN students sa ON sa.id = p.a
JOIN students sb ON sb.id = p.b
ORDER BY p.a, p.b;
```

## Detalles que importan

- **Una fila por pareja, no por curso.** La autounión da una fila por cada curso que comparten dos alumnos. Sin `DISTINCT`, dos alumnos con dos cursos en común salen dos veces: es el error de unir los nombres directamente sobre la autounión y ordenar por `e1.student_id, e2.student_id`.
- **`<`, no `<>`.** `e1.student_id <> e2.student_id` quita las parejas de un alumno consigo mismo, pero deja a la vez `(ana, bo)` y `(bo, ana)`. Con `<` estricto queda una sola orientación, la que pide el resultado (id menor primero), y nadie se encuentra consigo mismo.
- **Ordenar por ids, no por nombres.** El id menor no siempre es del nombre que va antes en el alfabeto, así que `ORDER BY student_a, student_b` puede dar un orden equivocado. Mantener los ids en la subconsulta permite usar `ORDER BY p.a, p.b` sin mostrarlos.
- **Unir `students` dos veces.** Cada lado de la pareja necesita su propia copia de la tabla (`sa` y `sb`); con una sola unión solo se puede nombrar a uno de los dos.
- **Los alumnos sin cursos** nunca aparecen en la autounión, así que no necesitan un caso aparte.

## Coste

La autounión compara las matrículas dentro de cada curso: un curso con `k` alumnos da unas `k²/2` filas. Un índice en `enrollments(course)` encuentra a los compañeros directamente; después se ordenan las parejas distintas y cada nombre es una búsqueda por clave primaria.
