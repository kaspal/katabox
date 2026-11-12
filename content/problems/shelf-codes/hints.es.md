Contar los números menores uno a uno son hasta 10⁵ pasos por consulta, y puede haber 10⁵ consultas. La lista está ordenada: ¿qué te dice eso sobre dónde están los números menores?
---
Todos los números menores que `x` forman un prefijo de la lista. La respuesta es la posición del primer número que es **al menos** `x` (o la longitud de la lista si no hay ninguno).
---
Busca esa posición con búsqueda binaria: mantén un rango `[lo, hi)` que contenga la primera posición con `codes[i] >= x`, mira el centro y quédate con la mitad que todavía la contiene. Unos 17 pasos por consulta.
