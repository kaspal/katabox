Hacer una búsqueda binaria por cada pedido cuesta O(n log m) por pedido, unos
10⁵ · 17 · 10⁵ pasos en total. Los rollos no cambian entre pedidos: ¿qué podrías
calcular una sola vez y reutilizar para todos?
---
Sea `count[L]` el número de trozos de longitud `L`. Solo baja a medida que `L` crece, y
los rollos miden como mucho 10⁵, así que puedes permitirte conocerlo para cada `L`.
Entonces cada pedido es una búsqueda binaria sobre `count`.
---
Un rollo de longitud `r` da un trozo de longitud `L` por cada múltiplo `k·L ≤ r`, así
que `count[L]` es la suma, para `k = 1, 2, …`, de cuántos rollos miden al menos `k·L`.
Un conteo prefijo de las longitudes responde cada término en O(1), y sumar sobre `L` da
m/1 + m/2 + … ≈ m ln m términos.
