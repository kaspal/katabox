## Idea

Las funciones de agregado de SQL ya se saltan los `NULL`, que es justo la regla que pide el café:

1. **Agrupar** las reseñas por juego.
2. **Contar y promediar** las estrellas, dejando que `COUNT(stars)` y `AVG(stars)` ignoren las reseñas que no las tienen.
3. **Redondear** la media y ordenar por juego.

## La consulta

```sql
SELECT game,
       COUNT(stars) AS rated,
       ROUND(AVG(stars), 2) AS avg_stars
FROM reviews
GROUP BY game
ORDER BY game;
```

## Detalles que importan

- **Una puntuación que falta no es un cero.** `AVG(COALESCE(stars, 0))` trata un comentario como una reseña de 0 estrellas: en el ejemplo 1, Hive baja de 4.5 a 3.0 y Azul muestra 0.0 en lugar de `NULL`. Dejar `stars` tal cual permite que `AVG` se salte los `NULL`.
- **`COUNT(stars)` frente a `COUNT(*)`.** `COUNT(*)` cuenta filas, así que daría 3 a Hive; `COUNT(columna)` cuenta solo los valores que no son `NULL`, que es lo que significa `rated`.
- **Grupos solo con `NULL`.** Un juego aparece porque tiene filas, aunque ninguna tenga estrellas. Entonces `COUNT(stars)` es 0 y `AVG(stars)` es `NULL`, y `ROUND(NULL, 2)` sigue siendo `NULL`, como se pide, sin necesidad de `CASE`.
- **Redondeo.** `AVG` devuelve un número real; `ROUND(x, 2)` deja dos decimales y redondea un medio alejándose de cero (3.125 pasa a 3.13). Una media de 2 sale como el real `2.0`, como en el ejemplo 2.
- **Orden.** Después de agrupar, los juegos son distintos, así que `ORDER BY game` es un orden completo.

## Coste

Una sola pasada de agrupación: SQLite ordena las reseñas por juego (o usa un índice temporal) y lleva un recuento y una suma por grupo, más o menos `O(n log n)` para `n` reseñas.
