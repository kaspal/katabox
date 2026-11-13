Primero averigua cuánto mide el tren: las longitudes de las partes solo dependen de n y de k.
---
Con n vagones y k partes, cada parte recibe n // k vagones, y las primeras n mod k partes reciben uno más.
---
Recorre la lista otra vez: para cada parte, guarda su primer nodo, avanza `tamaño − 1` veces hasta su último nodo, corta el enlace siguiente y continúa. Una parte vacía es `null`.
