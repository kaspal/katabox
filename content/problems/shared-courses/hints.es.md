Una pareja necesita dos matrículas en el mismo curso. ¿Qué obtienes si unes `enrollments` consigo misma por `course`?
---
`JOIN enrollments e2 ON e2.course = e1.course AND e1.student_id < e2.student_id` deja cada pareja una vez por curso compartido, con el id menor a la izquierda, y descarta a cada alumno emparejado consigo mismo. Pero dos cursos compartidos siguen dando dos filas.
---
Reduce primero a parejas: `SELECT DISTINCT e1.student_id AS a, e2.student_id AS b` en una subconsulta. Después une `students` dos veces, una para `a` y otra para `b`, llama a las columnas `student_a` y `student_b` y termina con `ORDER BY p.a, p.b`.
