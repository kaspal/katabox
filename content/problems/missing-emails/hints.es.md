Un correo puede faltar de tres maneras, y no se parecen: una ni siquiera es una cadena. ¿Puede una sola comparación detectar un `NULL`?
---
`email = NULL` nunca es verdadero, ni siquiera cuando `email` es `NULL`; para un valor ausente se usa `email IS NULL`. Para los otros dos casos, quita antes los espacios: `trim(email)` elimina los espacios del principio y del final, así que tanto `''` como `'   '` quedan en `''`.
---
`WHERE email IS NULL OR trim(email) = ''` deja justo a los lectores sin correo utilizable. Selecciona `id, name, city` y termina con `ORDER BY city, name, id`.
