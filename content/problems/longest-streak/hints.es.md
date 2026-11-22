¿En qué parte del arreglo está la racha más larga en las entradas que fallan? Prueba `[1, 2, 2]` y un arreglo de un solo elemento como `[5]`.
---
La longitud de la racha se cuenta bien. Fíjate en cuándo se actualiza `best` y qué pasa con la última racha.
---
`best` solo se actualiza cuando un valor distinto corta la racha, así que la racha final nunca se tiene en cuenta. Después del bucle, devuelve `max(best, run)` en lugar de `best`.
