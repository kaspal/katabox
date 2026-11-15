Subir desde `b` de jefe en jefe responde una pregunta, pero el organigrama puede ser una sola línea de 10⁵ personas y hay hasta 10⁵ preguntas. ¿Qué podrías numerar una sola vez, antes de las preguntas, para que cada respuesta sea una comparación?
---
Recorre el organigrama en profundidad desde el director y dale a cada empleado un número de entrada cuando lo alcances por primera vez. Todos los que están por debajo de `x` se visitan después de `x` y antes de que la búsqueda salga del equipo de `x`, así que sus números forman un único rango sin huecos.
---
Guarda `first[x]` (el número de entrada de `x`) y `last[x]` (el mayor número de entrada del equipo de `x`). Entonces `a` está por encima de `b` exactamente cuando `a ≠ b` y `first[a] ≤ first[b] ≤ last[a]`. Haz la búsqueda con una pila explícita, no con recursión: el organigrama puede tener 10⁵ niveles.
