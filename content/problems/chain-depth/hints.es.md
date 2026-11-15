Cada persona está exactamente un nivel más abajo que su jefe. Sabes dónde empieza esa regla (la directora, con 0), pero no cuántos niveles hay. ¿Cómo puede una consulta repetir el mismo paso hasta quedarse sin personas?
---
Un `WITH RECURSIVE chain(id, depth) AS (...)` tiene dos partes unidas con `UNION ALL`: las filas de partida, `SELECT id, 0 FROM staff WHERE boss_id IS NULL`, y un paso que añade a las personas cuyo `boss_id` es un id que ya está en `chain`, un nivel más abajo. SQLite repite el paso hasta que no añade filas nuevas.
---
El paso es `SELECT s.id, chain.depth + 1 FROM staff s JOIN chain ON s.boss_id = chain.id`. Después del `WITH`, une `chain` otra vez con `staff` por `id` para tener los nombres y termina con `ORDER BY chain.depth, s.name, s.id`.
