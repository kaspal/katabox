## Idea

Una tabla, un filtro y una ordenación. El único trabajo de verdad está en el filtro, porque «falta» abarca dos tipos de valor distintos:

1. **Ningún valor**: la columna es `NULL`.
2. **Un valor en blanco**: una cadena que queda vacía al quitarle los espacios.

## La consulta

```sql
SELECT id, name, city
FROM readers
WHERE email IS NULL OR trim(email) = ''
ORDER BY city, name, id;
```

## Detalles que importan

- **`= NULL` no coincide con nada.** Cualquier comparación con `NULL` da `NULL`, que `WHERE` trata como «no verdadero». Así, `email = NULL` descarta a todos los lectores que no tienen correo, justo a los que la editora más quiere llamar. Para un valor ausente se usa `IS NULL`.
- **Los espacios cuentan como blanco.** `email = ''` encuentra la cadena vacía pero no `'   '`. `trim(email)` con un solo argumento quita los espacios de ambos extremos, con lo que cualquier valor hecho solo de espacios queda en `''`. (Solo quita espacios; aquí no puede aparecer otro carácter en blanco).
- **El orden de las dos condiciones.** Con un correo `NULL`, `trim(email) = ''` también da `NULL`, pero un `OR` con `email IS NULL` verdadero sigue siendo verdadero, así que da igual en qué orden escribas las dos condiciones.
- **Un orden completo.** Dos lectores pueden tener el mismo nombre y la misma ciudad, así que el `id` deshace el último empate.

## Coste

Una pasada por los lectores para comprobar cada correo y luego ordenar las filas que quedan.
