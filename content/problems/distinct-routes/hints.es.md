Antes de comparar nada, limpia cada nombre. ¿Qué dos funciones de SQLite quitan los espacios de alrededor y pasan todo a una misma caja?
---
`lower(trim(origin))` y `lower(trim(dest))` son los nombres limpios; descarta las filas en las que coinciden. Aun así, Lima-Quito y Quito-Lima siguen pareciendo distintas: ¿cómo escribes cada pareja siempre en el mismo orden?
---
Con dos o más argumentos, `min(x, y)` y `max(x, y)` son funciones normales (no de agregado) que devuelven el valor menor y el mayor. Selecciona `min(...) AS city_a` y `max(...) AS city_b` de los dos nombres limpios, añade `DISTINCT` y termina con `ORDER BY city_a, city_b`.
