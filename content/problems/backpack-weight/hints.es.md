Probar todos los subconjuntos son 2²⁰⁰ opciones. Elegir con avaricia por valor por peso puede fallar (ejemplo 1). ¿Qué pregunta más pequeña podrías responder para cada capacidad?
---
Sea `best[c]` el mayor valor que cabe en la capacidad `c` usando los objetos vistos hasta ahora. ¿Cómo cambia `best` al añadir un objeto más?
---
Para cada objeto (peso w, valor v), para `c` desde `capacity` **hacia abajo** hasta `w`: `best[c] = max(best[c], best[c − w] + v)`. Ir hacia abajo asegura que cada objeto se usa como mucho una vez.
