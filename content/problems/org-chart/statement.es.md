Una empresa guarda su organigrama como una lista de jefes. El empleado `1` es el director y no tiene jefe; cualquier otro empleado tiene exactamente un jefe, y subiendo de jefe en jefe siempre se llega al director. El empleado `a` está **por encima** del empleado `b` cuando `b` depende de `a`, directamente o a través de una cadena de jefes. Nadie está por encima de sí mismo.

La oficina recibe muchas preguntas de la forma «¿está `a` por encima de `b`?». Respóndelas todas.

Este es un **problema de programa**: lee el organigrama y las preguntas de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `n`, el número de empleados, numerados de `1` a `n`.
- La segunda línea contiene `n − 1` números, separados por espacios: los jefes de los empleados `2`, `3`, …, `n`, en ese orden. Un jefe puede tener un número mayor que el del empleado. Cuando `n` es 1, esta línea está vacía.
- La tercera línea contiene `Q`, el número de preguntas.
- Después, `Q` líneas, cada una `a b`: ¿está `a` por encima de `b`?

## Formato de salida

Escribe `Q` líneas, una por pregunta y en orden: `YES` si `a` está por encima de `b`, y `NO` en caso contrario. Cuando `a` es igual a `b`, la respuesta es `NO`.

### Ejemplo 1

Entrada:

```
6
1 1 2 2 4
5
1 6
2 6
3 6
6 2
4 4
```

Salida:

```
YES
YES
NO
NO
NO
```

El empleado `6` depende de `4`, que depende de `2`, que depende del director `1`: así que `1` y `2` están por encima de `6`, pero `3` no. `6` está por debajo de `2`, no por encima, y `4` no está por encima de sí mismo.

### Ejemplo 2

Entrada:

```
1

1
1 1
```

Salida:

```
NO
```

El director trabaja solo, así que la línea de jefes está vacía.

### Ejemplo 3

Entrada:

```
5
4 1 5 3
4
3 2
4 2
2 3
1 5
```

Salida:

```
YES
YES
NO
YES
```

El organigrama es una sola línea: `2` depende de `4`, `4` de `5`, `5` de `3` y `3` de `1`.

### Restricciones

- 1 ≤ n ≤ 10⁵
- 1 ≤ jefe ≤ n, y nadie es su propio jefe
- Todos dependen al final del empleado `1`: el organigrama no tiene ciclos
- 1 ≤ Q ≤ 10⁵
- 1 ≤ a, b ≤ n
