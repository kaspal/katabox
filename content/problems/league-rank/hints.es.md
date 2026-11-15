Cada equipo conserva su fila, pero su puesto depende de los demás equipos de su división. ¿Qué tipo de función calcula un valor por fila a partir de un grupo de filas, sin juntarlas?
---
Una función de ventana de clasificación sobre `PARTITION BY division ORDER BY points DESC` numera cada división desde arriba. SQLite tiene tres: `ROW_NUMBER` nunca comparte número, `RANK` lo comparte en los empates pero luego salta (1, 1, 3) y `DENSE_RANK` lo comparte sin saltar (1, 1, 2).
---
Selecciona `division, name, points, DENSE_RANK() OVER (PARTITION BY division ORDER BY points DESC) AS place` de `teams` y termina con `ORDER BY division, place, name, id`.
