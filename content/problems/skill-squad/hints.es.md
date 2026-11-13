Probar cada equipo supone hasta 2⁶⁰ equipos. Pero lo único que importa de un equipo a medias es **qué habilidades** cubre ya. ¿Cuántos conjuntos "cubiertos" distintos hay?
---
Como mucho 2^skills = 16 384. Piensa en cada conjunto cubierto (una máscara) como un nodo; añadir al voluntario `p` lleva de `mask` a `mask | people[p]`.
---
El menor número de voluntarios para llegar a la máscara completa es un camino mínimo en este grafo: un BFS desde la máscara 0 en el que cada paso añade un voluntario. Quita primero los voluntarios repetidos; la respuesta es la distancia a `2^skills − 1`, o −1 si nunca se alcanza.
