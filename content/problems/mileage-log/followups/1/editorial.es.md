## Idea

Responde las preguntas **fuera de línea**, en el orden que nos conviene. Ordena las
preguntas por `x` de mayor a menor, y los días por kilómetros de mayor a menor. Guarda un
árbol de Fenwick sobre las posiciones de los días, todo a cero al principio.

Para cada pregunta en ese orden, suma primero 1 en la posición de cada día aún no añadido
cuyos kilómetros lleguen al menos a `x` (un puntero avanza por los días ordenados, así que
cada día se añade una vez). Ahora el árbol tiene un 1 justo en los días largos de este
umbral, y la respuesta es la suma de las posiciones `a` a `b`: dos sumas prefijas.
Escríbela en el índice original de la pregunta.

Como los kilómetros no pasan de 1000, también vale otro camino: dividir los días en
bloques de unos √n y guardar, por bloque, una cuenta para cada valor de kilómetros. También
es muchísimo más rápido que contar día a día.

## Complejidad

O((n + q) log n) de tiempo con la ordenación y el árbol de Fenwick, O(n + q) de memoria
extra. Contar día a día es O(n · q), hasta 10¹⁰ pasos.
