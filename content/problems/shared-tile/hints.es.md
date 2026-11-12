Una baldosa de lado `s` cubre el suelo exactamente cuando `s` divide a la vez a `width` y a `height`.
---
Buscas el máximo común divisor de los dos lados. Probar todos los tamaños desde el lado menor hacia abajo es demasiado lento para 10¹².
---
Algoritmo de Euclides: `mcd(a, b) = mcd(b, a mod b)` y `mcd(a, 0) = a`. Bastan unas pocas decenas de pasos incluso con 10¹².
