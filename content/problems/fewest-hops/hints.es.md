Cada salto cuesta lo mismo, así que la pregunta es por el número de enlaces de la ruta, no por su longitud. ¿Qué estaciones se alcanzan en un salto? ¿Y en dos?
---
Explora por anillos: primero la estación 0, luego todos sus vecinos (1 salto), luego todos los vecinos no visitados de estos (2 saltos), y así. El primer anillo que contiene la estación `n − 1` da la respuesta.
---
Eso es la búsqueda en anchura: una cola de estaciones y un arreglo `dist` que se fija la primera vez que se llega a una estación (`-1` = todavía no). Cada estación entra una vez en la cola, así que cuesta O(n + enlaces).
