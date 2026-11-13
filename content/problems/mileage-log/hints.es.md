Sumar los días de cada viaje uno a uno funciona, pero 10⁵ viajes de 10⁵ días es demasiado. ¿Qué podrías calcular una vez y reutilizar?
---
Si conocieras el total desde el día 0 hasta cualquier día, ¿podrías obtener el total de los días a a b con dos de esos números?
---
Construye `prefix[i]` = total de los días 0 a i − 1 (con `prefix[0] = 0`). Un viaje [a, b] es `prefix[b + 1] − prefix[a]`.
