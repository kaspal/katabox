## Idea

Un descuadre puede estar en cualquiera de los dos lados, así que la consulta tiene que mirar desde las dos tablas a la vez:

1. **Unión externa completa** de `bank` y `books` por `ref`: cada referencia de cualquiera de las tablas da una fila, con `NULL` en el lado donde falta.
2. **Filtrar** las filas que no cuadran: ausentes en un lado o con importes distintos.
3. **Tomar la referencia** del lado que la tenga y ordenar por ella.

## La consulta

```sql
SELECT COALESCE(b.ref, k.ref) AS ref,
       b.amount_cents AS bank_cents,
       k.amount_cents AS books_cents
FROM bank b
FULL OUTER JOIN books k ON k.ref = b.ref
WHERE b.ref IS NULL OR k.ref IS NULL OR b.amount_cents <> k.amount_cents
ORDER BY COALESCE(b.ref, k.ref);
```

## Detalles que importan

- **Mirar desde un solo lado.** Un `LEFT JOIN` desde `bank` encuentra los pagos que faltan en los libros y los importes distintos, pero un pago que solo está en los libros no tiene fila en `bank` desde la que partir, así que se pierde sin avisar. Con la tabla `bank` vacía, esa consulta no devuelve nada. La unión externa completa conserva las filas sin pareja de las dos tablas.
- **`NULL` nunca es distinto.** Cuando falta un lado, `b.amount_cents <> k.amount_cents` compara con `NULL` y no es verdadero, así que por sí sola descartaría justo las referencias de un solo lado. Las condiciones explícitas `b.ref IS NULL OR k.ref IS NULL` las conservan. Comprobar `ref` es seguro porque es `NOT NULL` en las dos tablas: un `NULL` ahí solo puede venir de la unión.
- **La columna de referencia.** En una fila de un solo lado, `b.ref` o `k.ref` es `NULL`; `COALESCE(b.ref, k.ref)` elige la que existe, para el resultado y para el orden.
- **Emparejar por `ref`, no por `id`.** Las dos tablas numeran sus filas por separado, como muestra el ejemplo 2.
- **Sin `FULL OUTER JOIN`.** El mismo resultado sale de un `LEFT JOIN` desde `bank` (como el de arriba) unido con `UNION ALL` a las filas de `books` cuya `ref` está `NOT IN (SELECT ref FROM bank)`, con `NULL` como `bank_cents`, y ordenando por `ref` al final. Como cada referencia aparece como mucho una vez por tabla, ninguna puede salir dos veces.

## Coste

Con un índice en `ref` en cualquiera de los lados, cada fila de una tabla encuentra su pareja en unos `O(log n)` pasos, y las filas sin pareja del otro lado se encuentran igual; después se ordenan los descuadres por `ref`. Sin índice, SQLite crea uno temporal, con un coste parecido.
