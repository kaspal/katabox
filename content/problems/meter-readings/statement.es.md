Un refugio de montaña se abastece de agua de un manantial, y un pequeño caudalímetro en la tubería registra una lectura por minuto durante toda la temporada. Cuando el refugio cierra, el guarda descarga el registro y quiere sacar tres números: la suma de todas las lecturas, cuántas veces una lectura fue mayor que la inmediatamente anterior y la longitud del tramo estable más largo, en el que la misma lectura se repite minuto tras minuto.

Este es un **problema de programa**: lee el registro de la entrada estándar y escribe los tres números en la salida estándar. El registro puede ser largo, así que léelo de forma eficiente.

## Formato de entrada

La primera línea contiene `N`, el número de lecturas. Cada una de las `N` líneas siguientes contiene una lectura, un número entero.

## Formato de salida

Escribe una línea con tres números separados por espacios:

1. la suma de todas las lecturas;
2. cuántas lecturas son estrictamente mayores que la lectura justo anterior;
3. la longitud de la racha más larga de lecturas consecutivas iguales (una sola lectura es una racha de longitud 1).

### Ejemplo 1

Entrada:

```
6
5
7
7
7
3
8
```

Salida:

```
37 2 3
```

Las lecturas suben dos veces (de 5 a 7 y de 3 a 8), y el 7 se repite tres veces seguidas.

### Ejemplo 2

Entrada:

```
1
1000000
```

Salida:

```
1000000 0 1
```

### Ejemplo 3

Entrada:

```
5
4
3
2
1
0
```

Salida:

```
10 0 1
```

### Ejemplo 4

Entrada:

```
4
0
0
1
1
```

Salida:

```
2 1 2
```

### Restricciones

- 1 ≤ N ≤ 6·10⁴
- 0 ≤ lectura ≤ 10⁶
