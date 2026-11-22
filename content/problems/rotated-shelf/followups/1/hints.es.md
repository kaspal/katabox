Subir desde 1 saltándose los números de la estantería responde a una pregunta, pero con
10⁵ preguntas sobre 10⁵ carpetas es demasiado lento. Olvida el giro un momento: en una
estantería ordenada, ¿cuántos números por debajo de la carpeta `i`-ésima están libres?
---
En la estantería ordenada `s`, la carpeta `i`-ésima (desde 0) tiene `i` números ocupados
por debajo, así que hay `s[i] − (i + 1)` números libres por debajo de `s[i]`. Esa cuenta
nunca baja al crecer `i`, así que se puede buscar con búsqueda binaria.
---
Encuentra el punto de giro con la búsqueda del problema base (u ordena una copia). Para cada
`k`, busca con búsqueda binaria el primer `i` con `s[i] − (i + 1) ≥ k`; la respuesta es
`k + i` (`i = n` cuando ninguna carpeta tiene tantos números libres por debajo).
