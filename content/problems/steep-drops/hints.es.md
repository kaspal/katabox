Comprobar cada par son unos 2·10¹⁰ comprobaciones. Supón que la primera mitad y la segunda mitad de los días estuvieran ordenadas cada una: ¿podrías contar rápido las caídas bruscas **entre** las dos mitades?
---
Con las dos mitades ordenadas, recorre la primera en orden creciente y avanza un puntero por la segunda: el número de precios `y` de la segunda mitad con `x > 2y` solo crece a medida que crece `x`.
---
Ordenación por mezcla: cuenta dentro de cada mitad de forma recursiva, cuenta entre las mitades con el puntero que avanza y después mezcla las mitades para que quien llamó reciba una lista ordenada. Cuenta antes de mezclar, porque la mezcla pierde de qué mitad venía cada precio.
