Los nombres están en una tabla y los préstamos en otra. ¿Cómo pones cada préstamo junto al socio que lo pidió?
---
`JOIN members m ON m.id = l.member_id` da a cada préstamo su socio. Las fechas escritas `AAAA-MM-DD` se comparan bien como texto, así que `l.returned > l.due` deja justo los tardíos; una fecha de devolución `NULL` nunca pasa esa condición.
---
`julianday(d)` de SQLite convierte una fecha en un número de día, así que `julianday(l.returned) - julianday(l.due)` son los días de retraso; envuélvelo en `CAST(... AS INTEGER)`. Llama a la columna `days_late` y termina con `ORDER BY days_late DESC, l.title, l.id`.
