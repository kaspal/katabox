Primero quita los inicios de sesión repetidos el mismo día. Después numera los días de cada alumno 1, 2, 3, … por orden de fecha. Dentro de una serie de días consecutivos, la fecha y el número suben de uno en uno a la vez. ¿Qué se mantiene igual?
---
`SELECT DISTINCT username, day FROM logins` deja una fila por alumno y día. Numera esas filas con `ROW_NUMBER() OVER (PARTITION BY username ORDER BY day)`. Entonces `julianday(day) - ROW_NUMBER() OVER (...)` tiene el mismo valor en todos los días de una racha y un valor nuevo después de cada hueco: sirve de etiqueta para las rachas.
---
Usa esa diferencia como clave de grupo: `GROUP BY username, grp` con `COUNT(*)` da la longitud de cada racha, y un segundo `GROUP BY username` con `MAX(...) AS longest` se queda con la más larga. Encadena los tres pasos con `WITH` y termina con `ORDER BY longest DESC, username`.
