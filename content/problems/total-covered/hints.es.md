Prueba un turno largo que contenga uno corto, como `shifts = [[1, 10], [2, 3]]`. ¿Qué turno ve primero el bucle que los une?
---
La unión solo funciona si, cuando llega un turno, ningún turno posterior puede empezar antes que el bloque actual. Revisa cómo se ordenan los turnos antes del bucle.
---
Los turnos se ordenan por su hora de fin, así que un turno que empieza antes puede llegar cuando el bloque ya fijó su inicio, y el tiempo anterior a ese inicio se pierde. Ordena por hora de inicio: basta con `sorted(shifts)` (en TypeScript, compara `x[0] - y[0]`).
