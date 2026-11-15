## Idea

El saldo después de cada movimiento es una suma acumulada, que es justo para lo que sirven las funciones de ventana:

1. **Particionar** los movimientos por cuenta, para que el saldo de cada una empiece en 0.
2. **Ordenar** los movimientos de cada cuenta por día y luego por id: el orden en que se aplican.
3. **Sumar** desde la primera fila de la partición hasta la fila actual.

## La consulta

```sql
SELECT account, day, amount_cents,
       SUM(amount_cents) OVER (
         PARTITION BY account
         ORDER BY day, id
         ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
       ) AS balance_cents
FROM transactions
ORDER BY account, day, id;
```

## Detalles que importan

- **Movimientos del mismo día.** Con `OVER (PARTITION BY account ORDER BY day)` y sin marco, SQLite usa el marco por defecto `RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW`, que incluye a los *pares* de la fila actual: todas las filas con el mismo `day`. Las dos retiradas de AC001 del 2026-02-03 en el ejemplo 1 mostrarían entonces el saldo de cierre del día, 6500, en vez de 7500 y 6500. Añadir `id` al orden de la ventana elimina los empates, y `ROWS` hace que el marco termine justo en la fila actual.
- **El orden de la ventana no es el del resultado.** El `ORDER BY` dentro de `OVER` decide qué cubre cada suma; el `ORDER BY account, day, id` final decide cómo se listan las filas. Aquí coinciden, pero hacen falta los dos.
- **Los ids no son cronológicos.** Un movimiento con un id mayor puede ser de un día anterior, así que ordenar solo por `id` da saldos incorrectos; el día va primero.
- **Sin funciones de ventana.** También sirve una subconsulta correlacionada: para cada fila `t`, sumar los importes de la misma cuenta con `day < t.day OR (day = t.day AND id <= t.id)`. Da el mismo resultado, pero vuelve a recorrer la cuenta para cada fila.

## Coste

SQLite ordena una vez los `n` movimientos por cuenta, día e id, y luego los recorre llevando una suma por partición: `O(n log n)` para ordenar y `O(n)` para las sumas. La subconsulta correlacionada cuesta unos `O(n²)` en el peor caso.
