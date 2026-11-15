## Idea

Es una **autounión** (self-join): la misma tabla cumple dos papeles. Una copia de `staff` (alias `s`) es la persona, la otra (alias `m`) es el mentor, y `m.id = s.mentor_id` las enlaza. Como hay gente sin mentor, la unión tiene que ser un `LEFT JOIN`, que conserva cada fila de `s` aunque ninguna `m` encaje.

## La consulta

```sql
SELECT s.name, m.name AS mentor
FROM staff s
LEFT JOIN staff m ON m.id = s.mentor_id
ORDER BY s.name, s.id;
```

## Detalles que importan

- **Una unión interna pierde gente.** Con un `JOIN` normal, una persona cuyo `mentor_id` es `NULL` no encaja con ninguna fila (`m.id = NULL` nunca es verdadero), así que Ada en el Ejemplo 1 y Eli en el Ejemplo 2 desaparecerían. `LEFT JOIN` las conserva y deja `m.name` a `NULL`.
- **Las condiciones extra, fuera del `WHERE`.** Un filtro sobre `m` escrito en el `WHERE` (por ejemplo `WHERE m.name <> 'Ada'`) es falso en las filas con `NULL` y convierte sin avisar el `LEFT JOIN` en uno interno. Las condiciones sobre el mentor van en el `ON`.
- **Qué nombre es cuál.** Las dos copias tienen una columna `name`, así que hay que cualificar cada columna. La segunda columna del resultado debe llamarse `mentor`, y se ordena por el nombre de la persona (`s.name`), no por el del mentor.
- **Los nombres se repiten.** Dos personas pueden llamarse igual, así que `s.id` deshace el empate. Ordenar solo por `name` dejaría su orden al azar.
- **Mentorías mutuas, sin problema.** Si dos personas son mentoras la una de la otra, cada fila sigue buscando un único mentor, así que la unión nunca entra en bucle.

## Coste

`m.id` es la clave primaria, así que el mentor de cada persona se encuentra con una sola búsqueda en el índice: más o menos un paso por miembro del personal y luego ordenar por nombre.
