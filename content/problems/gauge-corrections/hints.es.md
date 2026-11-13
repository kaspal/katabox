Sumar `delta` a cada medidor de un rango largo es la parte lenta. ¿Puede una actualización de rango tocar solo sus dos extremos?
---
Piensa en un arreglo de diferencias: sumar `delta` a los medidores `a..b` es `+delta` en `a` y `−delta` en `b + 1`. La corrección total de un medidor es la suma prefija de las diferencias hasta él.
---
Guarda las diferencias en un árbol de Fenwick: `"add"` son dos actualizaciones puntuales, y `"read"` es `readings[i] + prefix(i + 1)`.
