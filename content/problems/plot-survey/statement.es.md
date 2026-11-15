Un registro de la propiedad guarda cada parcela como un polígono: las esquinas de su cerca, en orden alrededor de la parcela, con coordenadas enteras en metros. Una agrimensora que revisa un registro antiguo necesita dos cifras de cada parcela: su área y cuántos puntos de coordenadas enteras quedan estrictamente dentro (los sitios donde podría ir un poste de señalización sin tocar la cerca).

Este es un **problema de programa**: lee la parcela de la entrada estándar y escribe su área y sus puntos interiores en la salida estándar.

## Formato de entrada

- La primera línea contiene `n`, el número de esquinas.
- Después, `n` líneas, cada una `x y`: las esquinas en orden alrededor de la parcela, en sentido horario o antihorario. La cerca va de cada esquina a la siguiente, y de la última de vuelta a la primera.

## Formato de salida

Escribe una línea: el área con exactamente un decimal (siempre termina en `.0` o `.5`), un espacio y el número de puntos de coordenadas enteras estrictamente dentro de la parcela. Los puntos sobre la cerca no cuentan.

### Ejemplo 1

Entrada:

```
4
0 0
4 0
4 3
0 3
```

Salida:

```
12.0 6
```

Un rectángulo de 4 × 3: los puntos interiores tienen `x` de 1 a 3 e `y` de 1 a 2, seis en total.

### Ejemplo 2

Entrada:

```
3
0 0
5 0
0 3
```

Salida:

```
7.5 4
```

Los puntos interiores son (1, 1), (2, 1), (3, 1) y (1, 2).

### Ejemplo 3

Entrada:

```
6
0 0
0 5
2 5
2 2
6 2
6 0
```

Salida:

```
18.0 8
```

Una parcela en forma de L dada en sentido horario.

### Restricciones

- 3 ≤ n ≤ 1000
- -10⁶ ≤ x, y ≤ 10⁶
- Las esquinas son puntos distintos.
- La parcela es un polígono simple: los lados solo se tocan en la esquina que comparten dos lados vecinos, y un lado nunca vuelve hacia atrás sobre el anterior. Una esquina puede estar en línea recta entre sus vecinas.
- El área es positiva.
