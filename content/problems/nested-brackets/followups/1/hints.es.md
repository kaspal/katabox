Lee el texto de izquierda a derecha y lleva la cuenta de cuántos `(` siguen esperando su `)`. ¿Qué significa un `)` cuando no hay ninguno esperando?
---
Un `)` sin nada abierto solo se arregla insertando un `(` en algún lugar antes de él. Cada `(` que sigue abierto al final necesita un `)` insertado después.
---
Usa dos contadores: paréntesis abiertos e inserciones. Con `(` suma uno a los abiertos; con `)` resta uno si hay alguno y, si no, cuenta una inserción. La respuesta es las inserciones más los que sigan abiertos al final.
