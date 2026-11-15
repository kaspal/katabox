## Idea

Es el patrón clásico de **huecos e islas**: los días en que un alumno entró forman "islas" de días consecutivos separadas por huecos, y queremos el tamaño de la isla más grande.

El truco: numera los días distintos de cada alumno 1, 2, 3, … por orden de fecha. Dentro de una isla, cada día siguiente es un día natural más tarde *y* un número más alto, así que `día − número de fila` se mantiene constante. Un hueco hace avanzar la fecha más de un día mientras el número sigue subiendo de uno en uno, así que la diferencia salta a un valor nuevo. Esa diferencia es una etiqueta de la isla.

1. **Una fila por alumno y día**: quitar los inicios de sesión repetidos.
2. **Etiquetar** cada día con `julianday(day) − ROW_NUMBER()`.
3. **Contar** los días de cada grupo (alumno, etiqueta) y **quedarse con el mayor** de cada alumno.

Para ana_r en el ejemplo 1, los días 1, 2, 3 y 5 de enero reciben los números 1, 2, 3 y 4. Los tres primeros comparten etiqueta; el 5 de enero recibe una etiqueta una unidad mayor, así que es otra isla.

## La consulta

```sql
WITH days AS (
  SELECT DISTINCT username, day FROM logins
),
grouped AS (
  SELECT username,
         julianday(day) - ROW_NUMBER() OVER (PARTITION BY username ORDER BY day) AS grp
  FROM days
),
runs AS (
  SELECT username, COUNT(*) AS len FROM grouped GROUP BY username, grp
)
SELECT username, MAX(len) AS longest
FROM runs
GROUP BY username
ORDER BY longest DESC, username;
```

## Detalles que importan

- **Primero los días distintos.** Numerar cada inicio de sesión en lugar de cada día rompe el truco: un segundo inicio de sesión el mismo día se lleva un número de fila sin que avance la fecha, así que la diferencia baja en uno y la isla se parte en dos. Un alumno que entra el 1, el 2 (dos veces) y el 3 de enero recibe las etiquetas `x`, `x`, `x − 1`, `x − 1`: dos trozos de 2 días y una respuesta de 2 en lugar de 3. Días posteriores pueden incluso caer por casualidad en un trozo anterior. Contar después `COUNT(DISTINCT day)` por grupo no lo arregla, porque la isla ya está partida. Quita los duplicados *antes* de numerar.
- **Días naturales, no texto.** `julianday` convierte una fecha en un número de día que sigue sin saltos al cambiar de mes o de año, así que del 28 de febrero al 1 de marzo y del 31 de diciembre al 1 de enero hay un día. Tomar el día del mes con `strftime('%d', ...)` o comparar trozos del texto fallaría en esos cambios.
- **Etiquetas exactas.** `julianday` devuelve un número terminado en `.5` para una fecha sin hora, y restarle un número de fila entero lo deja exacto, así que agrupar por el resultado es seguro.
- **`PARTITION BY username`.** Cada alumno necesita su propia numeración desde 1; una sola numeración para todos mezclaría sus días.
- **Todos tienen al menos 1.** Cualquier alumno con un inicio de sesión tiene una isla de al menos un día, así que cada `username` de `logins` sale en el resultado.

## Coste

Quitar duplicados, numerar por alumno y las dos agrupaciones ordenan o recorren las filas una vez cada una: unos `O(n log n)` pasos para `n` inicios de sesión.
