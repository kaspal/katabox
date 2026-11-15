## Idea

El contacto es una cadena de alternativas, que es justo para lo que sirve `COALESCE`:

1. **El valor del canal preferido**, si hay preferencia y ese valor existe.
2. **El teléfono** y, si no, **el correo**.
3. **`'no contact'`**, que nunca es `NULL`, así que la cadena siempre termina en un valor.

Una vez que el contacto ya no puede ser `NULL`, construir la línea es una simple concatenación.

## La consulta

```sql
SELECT id,
       last_name || ', ' || first_name || ': ' ||
       COALESCE(CASE prefers WHEN 'phone' THEN phone WHEN 'email' THEN email END,
                phone, email, 'no contact') AS contact_line
FROM clients
ORDER BY last_name, first_name, id;
```

## Detalles que importan

- **La preferencia va primero.** `COALESCE(phone, email, 'no contact')` a secas ignora `prefers`: a un cliente que tiene los dos y prefiere el correo se le mostraría el teléfono. El `CASE` pone el valor preferido al principio de la cadena de alternativas.
- **Un `CASE` sin `ELSE` da `NULL`.** Cuando `prefers` es `NULL` no coincide ningún `WHEN` (un `NULL` no es igual a nada), y cuando el canal preferido es `NULL` el `CASE` también devuelve `NULL`. En ambos casos `COALESCE` pasa al teléfono y luego al correo, que es la alternativa que pidió el taller. Así, un cliente que prefiere el teléfono pero no lo tiene recibe su correo.
- **`||` con `NULL` da `NULL`.** Concatenar directamente un teléfono `NULL` convertiría toda la línea en `NULL`. Resolver antes el contacto dentro de `COALESCE`, con un último recurso que no es `NULL`, garantiza que cada línea sea una cadena.
- **Separadores exactos.** La línea es `last_name`, una coma y un espacio, `first_name`, dos puntos y un espacio, y luego el contacto: `Rios, Ana: ana@rios.test`.
- **Un orden completo.** Dos clientes pueden compartir nombre y apellido, así que el `id` deshace el último empate.

## Coste

Una pasada por los clientes para construir cada línea y luego ordenar por nombre.
