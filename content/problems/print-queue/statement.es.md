Una oficina tiene una sola impresora y una larga lista de trabajos de impresión. Los trabajos se numeran 1, 2, 3, … en el orden en que aparecen en la lista; cada uno llega en algún segundo, tiene una prioridad y un número de páginas, y la impresora imprime una página por segundo. Cuando un trabajo empieza, se imprime hasta el final sin interrupciones.

Cada vez que la impresora queda libre, elige entre los trabajos que ya han llegado y siguen esperando: el de **mayor prioridad**; entre ellos, el que **llegó antes**; entre ellos, el de **menor número**. Un trabajo que llega justo en el segundo en que la impresora queda libre ya está esperando. Si no hay nada esperando, la impresora se queda parada hasta que llega el siguiente trabajo.

Este es un **problema de programa**: lee los trabajos de la entrada estándar y escribe el orden en que se imprimen en la salida estándar.

## Formato de entrada

- La primera línea contiene `n`, el número de trabajos.
- Después, `n` líneas, cada una `arrival priority pages`: el segundo de llegada del trabajo, su prioridad (mayor es más urgente) y su número de páginas. La línea `i` describe el trabajo `i`. Las líneas **no** están ordenadas por llegada.

## Formato de salida

Escribe una línea con los `n` números de trabajo en el orden en que se imprimen, separados por espacios.

### Ejemplo 1

Entrada:

```
4
0 2 3
1 5 2
1 9 1
2 5 2
```

Salida:

```
1 3 2 4
```

En el segundo 0 solo ha llegado el trabajo 1, así que se imprime de 0 a 3. Para entonces esperan los trabajos 2, 3 y 4, y el 3 tiene la mayor prioridad (9); se imprime de 3 a 4. Los trabajos 2 y 4 tienen prioridad 5, y el 2 llegó antes.

### Ejemplo 2

Entrada:

```
2
10 1 5
0 1 5
```

Salida:

```
2 1
```

El trabajo 2 llega primero y se imprime de 0 a 5; después la impresora espera hasta que llega el trabajo 1 en el segundo 10.

### Ejemplo 3

Entrada:

```
3
5 7 2
5 7 2
5 7 2
```

Salida:

```
1 2 3
```

### Restricciones

- 1 ≤ n ≤ 5·10⁴
- 0 ≤ arrival ≤ 10⁹
- 1 ≤ priority ≤ 10⁹
- 1 ≤ pages ≤ 10⁴
