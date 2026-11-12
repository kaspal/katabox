El número de categorías no es fijo, así que no sirven bucles anidados (uno por categoría). Piensa en elegir el conjunto categoría a categoría.
---
Escribe una función recursiva `elegir(c, gastado)` que pruebe cada prenda de la categoría `c`, sume su precio y llame a `elegir(c + 1, gastado + precio)`. Cuando `c` pasa de la última categoría, hay un conjunto completo: cuéntalo si `gastado ≤ budget`.
---
Poda: los precios son positivos, así que en cuanto `gastado` pasa del presupuesto ninguna forma de completarlo cabe. Ordenar los precios de cada categoría permite cortar el bucle en la primera prenda demasiado cara.
