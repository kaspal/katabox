Con 15 platos como mucho hay como mucho 2¹⁵ = 32 768 conjuntos. ¿Cómo los recorres todos sin recursión?
---
Usa un número `mask` de 1 a 2ⁿ − 1 como conjunto: el plato `i` está en el conjunto cuando el bit `i` de `mask` es 1, es decir, cuando `mask & (1 << i)` no es 0.
---
Para cada máscara, suma los precios de sus platos; si el total es el presupuesto, recoge los números de plato (en orden creciente de `i`). Las máscaras no salen en orden lexicográfico, así que ordena las listas al final.
