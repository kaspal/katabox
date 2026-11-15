## Idea

Es un **anti-join**: conservar cada socio para el que no existe ninguna fila que encaje en otra tabla. Aquí la fila que encaja es "una visita de este socio en noviembre de 2026", así que la condición de fecha va dentro de la prueba de existencia, no fuera.

## La consulta

```sql
SELECT m.id, m.name
FROM members m
WHERE NOT EXISTS (
  SELECT 1 FROM visits v
  WHERE v.member_id = m.id AND v.day BETWEEN '2026-11-01' AND '2026-11-30'
)
ORDER BY m.name, m.id;
```

## Detalles que importan

- **Inactivo en noviembre, no inactivo siempre.** `WHERE m.id NOT IN (SELECT member_id FROM visits)` encuentra a los socios que no fueron nunca. Un socio cuya única visita fue en octubre (Bo en el Ejemplo 1) tiene una visita, así que esa consulta lo descarta, y sin embargo está inactivo en noviembre. El mes tiene que formar parte de la subconsulta.
- **Los dos extremos del mes cuentan.** `BETWEEN` incluye los dos límites, así que las visitas del 1 y del 30 de noviembre hacen activo a un socio, y las del 31 de octubre y el 1 de diciembre no. Con `< '2026-11-30'` se perdería el último día (Ejemplo 2).
- **Fechas como texto.** Cada `day` tiene la forma `AAAA-MM-DD`, así que el orden del texto es el de las fechas y `BETWEEN` sobre cadenas es seguro. `v.day LIKE '2026-11-%'` o `strftime('%Y-%m', v.day) = '2026-11'` también sirven.
- **Un `LEFT JOIN` también vale, con el filtro en su sitio.** `LEFT JOIN visits v ON v.member_id = m.id AND v.day BETWEEN ... WHERE v.id IS NULL` es equivalente. Si la condición de fecha pasa al `WHERE`, no sale ninguna fila: un socio sin coincidencia tiene `day` a `NULL`, que nunca cumple el `BETWEEN`.
- **`NOT IN` y `NULL`.** Si la lista interior de un `NOT IN` contiene un `NULL`, la condición nunca es verdadera y no sale ninguna fila. Aquí `member_id` es `NOT NULL`, pero `NOT EXISTS` no tiene esa trampa.
- **Los nombres se repiten.** Dos socios pueden llamarse igual, así que `m.id` deshace el empate.

## Coste

Para cada socio, SQLite busca una visita de noviembre y se detiene en la primera. `visits.member_id` no tiene índice propio, así que SQLite suele crear uno automático temporal; con él, la consulta hace más o menos una búsqueda por socio y una ordenación, del orden de (socios + visitas) log visitas.
