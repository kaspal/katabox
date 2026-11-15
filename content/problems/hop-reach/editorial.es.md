## Idea

Recorre la red desde `Central` una travesía cada vez, y para después de tres:

1. **Empezar** en `Central` con 0 travesías.
2. **Seguir** cada ruta que sale de un muelle ya alcanzado, con una travesía más, mientras se hayan usado menos de 3.
3. **Quedarse con el mínimo** de travesías por muelle, dejar fuera `Central` y ordenar el resultado.

Una expresión de tabla común recursiva hace el recorrido. Su primera parte es la fila de partida; la segunda toma cada fila encontrada hasta el momento y sigue las rutas que salen de ese muelle.

## La consulta

```sql
WITH RECURSIVE reach(pier, hops) AS (
  SELECT 'Central', 0
  UNION
  SELECT r.to_pier, reach.hops + 1
  FROM reach
  JOIN routes r ON r.from_pier = reach.pier
  WHERE reach.hops < 3
)
SELECT pier, MIN(hops) AS hops
FROM reach
WHERE pier <> 'Central'
GROUP BY pier
ORDER BY hops, pier;
```

## Detalles que importan

- **`< 3`, no `<= 3`.** La condición se refiere a la fila que se *extiende*: una fila con 2 travesías puede hacer una más y llegar a 3, pero una que ya tiene 3 debe parar. Con `reach.hops <= 3` también se extienden las filas con 3 travesías, así que un muelle que necesita cuatro travesías, como el último de `Central → A → B → C → D`, acaba en la lista con `hops = 4`.
- **El límite es lo que para los ciclos.** En el ejemplo 1, Central → Island → Central → Island … seguiría para siempre sin límite, y `UNION` por sí solo no lo detiene, porque cada vuelta tiene otro `hops` y es una fila nueva. La condición `hops < 3` termina cada recorrido tras tres travesías.
- **`UNION` lo mantiene pequeño.** Con `UNION ALL`, `reach` guarda una fila por cada recorrido de hasta tres travesías, y en una red densa el número de recorridos crece como el cubo del número de rutas por muelle. `UNION` descarta una fila `(pier, hops)` que ya está antes de volver a seguir sus rutas, así que `reach` tiene como mucho cuatro filas por muelle (de 0 a 3 travesías).
- **El menor número de travesías.** A un muelle se puede llegar por recorridos de distinta longitud (a Island, en el ejemplo 1, con 1 y con 2), así que `GROUP BY pier` con `MIN(hops)` se queda con el más corto.
- **`Central` no es un destino.** Una ruta de vuelta a `Central` lo mete otra vez en `reach`; el `WHERE pier <> 'Central'` final lo quita. Esa condición también quita la fila de partida, así que un `Central` sin rutas de salida, como en el ejemplo 2, da un resultado vacío.
- **Rutas de un solo sentido.** Solo se sigue `from_pier → to_pier`; una ruta que llega a `Central` no sirve para salir de él.

## Coste

`reach` tiene como mucho cuatro filas por muelle, y cada fila se une con las rutas que salen de su muelle. Con hasta 400 rutas eso son unos pocos miles de filas como mucho; sin índice en `from_pier`, cada unión recorre `routes`, y aun así son alrededor de un millón de comparaciones. La agrupación y la ordenación final cuestan `O(p log p)` para `p` muelles.
