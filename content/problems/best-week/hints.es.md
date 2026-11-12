Sumar cada tramo de `k` días desde cero cuesta O(k) por tramo. ¿Qué cambia entre un tramo y el siguiente?
---
Cuando el tramo avanza un día a la derecha, entra un día y sale otro.
---
Suma los primeros `k` días. Después, para cada día siguiente `i`, suma `sales[i]` y resta `sales[i - k]`, guardando el mayor total visto.
