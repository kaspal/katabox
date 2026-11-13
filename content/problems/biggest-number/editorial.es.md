## Enfoque: ordenar con la regla de "qué pareja se lee mayor"

Escribe cada carta como texto. Para dos cartas `a` y `b`, pon `a` primero cuando `a + b > b + a` (concatenación de cadenas; los dos resultados tienen la misma longitud, así que comparar cadenas es comparar números).

Esta regla es un orden de verdad: es coherente con tres o más cartas (si `a` debe ir antes que `b` y `b` antes que `c`, entonces `a` va antes que `c`), así que una ordenación produce una disposición en la que ningún intercambio de vecinos mejora, y ese es el número mayor.

Une los textos ordenados. Si el primer carácter es `0`, todas las cartas eran 0: devuelve `"0"`.

### Paso a paso

Ejemplo 3: `4` frente a `40`: "440" > "404", así que el 4 primero. `4` frente a `404`: "4404" > "4044", así que el 4 primero. `404` frente a `40`: "40440" > "40404", así que el 404 primero. El orden 4, 404, 40 da "440440".

**Complejidad:** O(n log n) comparaciones de cadenas de 20 caracteres como mucho.
