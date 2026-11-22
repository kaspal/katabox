## Idea

Extiende las sumas prefijas del problema base a una cuadrícula. Construye `P` con una fila
y una columna extra de ceros, donde `P[r][c]` es el total de todas las celdas de las filas
`0..r−1` y las columnas `0..c−1`. Cada entrada es su celda más el bloque de arriba más el
bloque de la izquierda, menos el bloque de la esquina, que ambos contaron. Un rectángulo
`[v1, d1, v2, d2]` es entonces el bloque grande `P[v2+1][d2+1]`, menos la franja de arriba
`P[v1][d2+1]`, menos la franja de la izquierda `P[v2+1][d1]`, más `P[v1][d1]`, que se restó
dos veces (inclusión-exclusión).

## Complejidad

O(V · D) en tiempo para construir la tabla con `V` furgonetas y `D` días, y luego O(1) por
pregunta; O(V · D) de memoria extra. Un total es como mucho 10⁵ · 1000 = 10⁸, que cabe de
sobra en un entero de 32 bits.
