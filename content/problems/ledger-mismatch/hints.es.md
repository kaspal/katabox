Un `JOIN` normal por `ref` solo deja las referencias que están en las dos tablas, y dos de los tres tipos de descuadre están en una sola. ¿Qué unión conserva las filas sin pareja de *los dos* lados?
---
`bank b FULL OUTER JOIN books k ON k.ref = b.ref` conserva cada referencia de cualquiera de los lados; el lado al que le falta queda entero a `NULL`. (Sin ella: un `LEFT JOIN` desde `bank` más un `UNION ALL` de las filas de `books` cuya `ref` no está en `bank`.) La referencia sale entonces del lado que la tenga: `COALESCE(b.ref, k.ref)`.
---
Conserva una fila cuando `b.ref IS NULL OR k.ref IS NULL OR b.amount_cents <> k.amount_cents`; las dos primeras condiciones hacen falta porque `<>` con un `NULL` nunca es verdadero. Selecciona `COALESCE(b.ref, k.ref) AS ref, b.amount_cents AS bank_cents, k.amount_cents AS books_cents` y ordena por `COALESCE(b.ref, k.ref)`.
