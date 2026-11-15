## Idea

Paginar son tres pasos, y su orden importa:

1. **Filtrar** los anuncios que muestra la web.
2. **Ordenarlos** de una forma exacta, para que cada anuncio tenga una posición fija.
3. **Recortar** las posiciones 21 a 30.

## La consulta

```sql
SELECT id, title, rating
FROM listings
WHERE hidden = 0
ORDER BY rating DESC, title, id
LIMIT 10 OFFSET 20;
```

## Detalles que importan

- **La página `p` salta `(p - 1) × 10` filas.** La página 3 salta las 20 filas de las páginas 1 y 2. Con `OFFSET 30` se saltan tres páginas y sale la página 4; es el clásico error de uno al paginar.
- **`LIMIT` y `OFFSET` se aplican después de `WHERE` y `ORDER BY`.** Los anuncios ocultos se quitan antes de contar, así que nunca ocupan un hueco, y el recorte se hace sobre la lista ordenada, no sobre el orden en que la tabla guarda las filas.
- **El orden tiene que ser total.** Si dos anuncios comparten valoración y título, solo el `id` decide en qué página cae cada uno. Sin él, un anuncio podría salir en dos páginas o en ninguna si la base de datos elige otro orden entre una petición y la siguiente.
- **Las páginas cortas o vacías son válidas.** `LIMIT 10` es un máximo: con 25 anuncios visibles la página 3 tiene 5 filas, y con 20 o menos no tiene ninguna.

## Coste

SQLite recorre los anuncios y ordena los visibles. Gracias al `LIMIT`, al ordenar solo necesita guardar las 30 mejores filas, pero aun así lee todas. Las páginas lejanas se vuelven más lentas con `OFFSET`, porque las filas saltadas se siguen produciendo y descartando.
