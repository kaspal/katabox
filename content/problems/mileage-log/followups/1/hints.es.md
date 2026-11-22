Las sumas prefijas del problema base cuentan todos los días, pero aquí cada pregunta tiene
su propio umbral `x`. Contar día a día son hasta 10⁵ pasos por pregunta. ¿Y si todas las
preguntas tuvieran el mismo `x`?
---
Con un solo umbral, marca los días largos con 1 y los demás con 0, y las sumas prefijas
responden a cualquier rango. Umbrales distintos solo significan que las marcas cambian. Si
respondes las preguntas de la `x` más grande a la más pequeña, cada día solo pasa de 0 a 1,
una vez.
---
Ordena los días por kilómetros y las preguntas por `x`, las dos cosas de mayor a menor.
Recorre las preguntas activando primero en un árbol de Fenwick (un 1 en su posición) todo
día con al menos `x` km, y responde con dos sumas prefijas: `sum(b) − sum(a − 1)`. Guarda
cada respuesta en el índice original de su pregunta.
