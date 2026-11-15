El nombre del mentor está en la misma tabla que el de la persona, solo que en otra fila. ¿Cómo puede una consulta leer `staff` dos veces?
---
Dale a la tabla dos alias: `staff s` para la persona y `staff m` para el mentor, emparejados con `m.id = s.mentor_id`. Pero un `JOIN` normal solo conserva las filas que encuentran pareja, y quien no tiene mentor no encuentra ninguna. ¿Qué tipo de unión las conserva?
---
`FROM staff s LEFT JOIN staff m ON m.id = s.mentor_id` conserva a todos y rellena las columnas del mentor con `NULL` cuando no hay pareja. Selecciona `s.name, m.name AS mentor` y termina con `ORDER BY s.name, s.id`.
