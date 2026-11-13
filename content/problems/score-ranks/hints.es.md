El puesto de una puntuación que alguien sacó es 1 más el número de puntuaciones **distintas** por encima. Contarlas en cada pregunta recorriendo a todos los alumnos es demasiado lento: 20 000 preguntas por 20 000 alumnos.
---
Los repetidos no cambian los puestos, así que quítalos. Con las puntuaciones distintas ordenadas de mayor a menor, el puesto de una puntuación es simplemente su posición en esa lista (contando desde 1).
---
Ordena las puntuaciones distintas de mayor a menor y, o bien busca cada puntuación preguntada con búsqueda binaria (puesto = índice + 1 si está, 0 si no), o bien construye una vez un mapa de puntuación a puesto y consulta cada pregunta en él. Escribe todas las respuestas en una sola línea.
