Probar todos los tríos es O(n³): unos 1300 millones de tríos con 2000 regalos. Fija primero el regalo más barato del trío; lo que queda es una pregunta sobre parejas.
---
Con el primer regalo fijo, busca entre los más caros la pareja cuya suma esté más cerca de `target − primero`. En una lista ordenada, dos punteros lo hacen en una pasada.
---
Ordena los precios. Para cada `i`, pon `lo = i + 1` y `hi = n − 1`. Si la suma del trío queda por debajo del objetivo, mueve `lo` a la derecha; si queda por encima, mueve `hi` a la izquierda; si es igual, ya está. Guarda la mejor suma, prefiriendo la menor en los empates.
