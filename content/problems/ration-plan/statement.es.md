El depósito de una expedición vende `n` paquetes de raciones, cada uno a su propio precio. Antes de partir, cada uno de los `Q` equipos llega con un presupuesto y quiere llevarse tantos paquetes como pueda. Un equipo puede comprar cada paquete como mucho una vez. La pregunta de cada equipo es independiente: las existencias del depósito son las mismas para todos los equipos, compren lo que compren los demás.

Este es un **problema de programa**: lee los precios y los presupuestos de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `n Q`: el número de paquetes de raciones y el número de equipos.
- La segunda línea contiene los `n` precios, separados por espacios.
- Después, `Q` líneas, cada una con el presupuesto de un equipo.

## Formato de salida

Escribe `Q` líneas, una por equipo y en orden: el mayor número de paquetes cuyos precios suman como mucho el presupuesto de ese equipo.

### Ejemplo 1

Entrada:

```
5 4
7 3 12 3 5
10
2
30
18
```

Salida:

```
2
0
5
4
```

Con un presupuesto de 10, un equipo puede comprar los dos paquetes de 3 (total 6); añadir el de 5 costaría 11. Un presupuesto de 18 compra los paquetes de 3, 3, 5 y 7, exactamente 18.

### Ejemplo 2

Entrada:

```
1 2
1000000000
999999999
1000000000
```

Salida:

```
0
1
```

### Ejemplo 3

Entrada:

```
4 3
2 2 2 2
0
7
8
```

Salida:

```
0
3
4
```

### Restricciones

- 1 ≤ n ≤ 5·10⁴
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ precio ≤ 10⁹
- 0 ≤ presupuesto ≤ 5·10¹³
