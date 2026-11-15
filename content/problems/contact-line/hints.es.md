Divide el trabajo en dos: primero calcula el contacto como un único valor y después pega los nombres y el contacto en una sola cadena. ¿Qué pasa si pegas un `NULL` a una cadena?
---
`'a' || NULL` es `NULL`, así que cada trozo que concatenes ya tiene que ser no `NULL`. `COALESCE(x, y, z)` devuelve su primer argumento que no es `NULL`, que es justo «esto, si no aquello, si no lo otro». El valor preferido es `CASE prefers WHEN 'phone' THEN phone WHEN 'email' THEN email END`, que da `NULL` cuando no hay preferencia.
---
Pon ese `CASE` el primero dentro de un `COALESCE`, seguido de `phone`, `email` y `'no contact'`. Concatena `last_name || ', ' || first_name || ': ' || COALESCE(...)`, llámalo `contact_line`, selecciónalo junto a `id` y termina con `ORDER BY last_name, first_name, id`.
