## Enfoque: simular con un bucle while

Pon un contador de pasos a 0. Mientras el número no sea 1, aplica la regla (divide entre dos un número par; convierte uno impar en `3n + 1`) y suma uno al contador. Cuando el bucle termina, el contador es la respuesta; para `n = 1` el bucle no llega a ejecutarse.

Los números intermedios pueden crecer mucho más que `n` (para n ≤ 10⁶ se quedan por debajo de 10¹¹), así que usa enteros de 64 bits donde el lenguaje los distinga.

**Complejidad:** proporcional al número de pasos (unos pocos cientos como mucho en este rango).
