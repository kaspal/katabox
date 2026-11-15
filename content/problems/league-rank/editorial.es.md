## Idea

Un puesto es una clasificación calculada dentro de un grupo, y eso es una función de ventana:

1. **Particionar** los equipos por división, para que cada una se clasifique por su cuenta.
2. **Clasificar** por puntos, de más a menos, con los empates compartiendo puesto y sin huecos después.
3. **Ordenar** el resultado para publicarlo.

## La consulta

```sql
SELECT division, name, points,
       DENSE_RANK() OVER (PARTITION BY division ORDER BY points DESC) AS place
FROM teams
ORDER BY division, place, name, id;
```

## Detalles que importan

- **`RANK` deja huecos.** `RANK()` da a una fila 1 más el número de filas estrictamente por delante, así que tras dos equipos empatados en el puesto 1 el siguiente queda en el puesto 3. En el ejemplo 1, United sería 3 y Town 4. La liga cuenta en cambio los totales de puntos distintos, que es `DENSE_RANK()`: uno más el número de totales distintos mayores.
- **`ROW_NUMBER` rompe los empates.** Numera 1, 2, 3, ... incluso con los mismos puntos, así que equipos que deberían compartir puesto no lo harían.
- **De más a menos.** `ORDER BY points DESC` dentro de `OVER` pone en el puesto 1 al que más puntos tiene; sin `DESC` ganaría el último.
- **Un orden completo.** Dos equipos de una división pueden tener los mismos puntos y el mismo nombre, así que `id` deshace el último empate. `ORDER BY` puede usar el alias `place`.
- **Sin funciones de ventana.** Una subconsulta correlacionada da el mismo número: `1 + (SELECT COUNT(DISTINCT u.points) FROM teams u WHERE u.division = t.division AND u.points > t.points)`. Vuelve a recorrer la división para cada equipo.

## Coste

SQLite ordena una vez los `n` equipos por división y puntos y luego los numera en una pasada: `O(n log n)`, más otra ordenación para el orden final. La subconsulta correlacionada cuesta unos `O(n²)` en el peor caso.
