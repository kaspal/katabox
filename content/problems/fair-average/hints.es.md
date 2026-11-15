Una fila por juego significa `GROUP BY game`. La cuestión es cómo tratan las funciones de agregado un `NULL` en `stars`. ¿Qué hacen con él `COUNT(*)`, `COUNT(stars)` y `AVG(stars)`?
---
`COUNT(stars)` cuenta solo los valores que no son `NULL`, y `AVG(stars)` hace la media solo de ellos, que es justo la media justa. Cuando un grupo no tiene ningún valor distinto de `NULL`, `AVG` devuelve `NULL` y `COUNT(stars)` devuelve 0.
---
`SELECT game, COUNT(stars) AS rated, ROUND(AVG(stars), 2) AS avg_stars FROM reviews GROUP BY game ORDER BY game`. No conviertas los `NULL` en 0 con `COALESCE`: eso cuenta un comentario como una reseña de 0 estrellas.
