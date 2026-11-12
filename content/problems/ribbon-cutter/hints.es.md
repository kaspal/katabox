Para una longitud fija `L`, ¿cuántos trozos salen? ¿Cómo cambia ese número cuando `L` crece?
---
La cuenta `sum(r // L)` nunca aumenta al crecer `L`. Así que las longitudes que sirven forman un prefijo 1, 2, …, respuesta. Probar las longitudes una a una puede suponer 10⁷ intentos.
---
Haz una búsqueda binaria de `L` entre 1 y el rollo más largo: si la longitud `mid` da al menos `pieces` trozos, la respuesta es `mid` o más; si no, es menos. Comprueba primero que la longitud 1 sirve.
