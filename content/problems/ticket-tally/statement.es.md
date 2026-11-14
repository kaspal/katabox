La oficina de un ferri lleva un registro de los billetes que vende, un día tras otro. Para cada día, la gerente quiere tres números: cuántos billetes se vendieron, los ingresos totales y el precio del billete más caro.

Este es un **problema de programa**: lee el registro de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

La primera línea contiene `T`, el número de días. Después, cada día ocupa dos líneas:

- una línea con `n`, el número de billetes vendidos ese día;
- una línea con los `n` precios, separados por espacios. Cuando `n` es 0, esta línea está vacía.

## Formato de salida

Escribe `T` líneas, una por día y en orden: `cantidad total máximo`, separados por un espacio. Un día sin billetes escribe `0 0 0`.

### Ejemplo 1

Entrada:

```
2
3
12 30 8
1
45
```

Salida:

```
3 50 30
1 45 45
```

### Ejemplo 2

Entrada:

```
3
0

2
7 7
4
5 1 9 2
```

Salida:

```
0 0 0
2 14 7
4 17 9
```

El primer día no se vendió nada: su línea de precios está vacía.

### Ejemplo 3

Entrada:

```
1
5
10000 10000 10000 10000 10000
```

Salida:

```
5 50000 10000
```

### Restricciones

- 1 ≤ T ≤ 1000
- 0 ≤ n ≤ 1000
- 1 ≤ precio ≤ 10 000
- Como mucho 50 000 precios en toda la entrada.
