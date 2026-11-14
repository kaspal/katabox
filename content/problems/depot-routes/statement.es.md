Todas las ambulancias de una ciudad esperan en una misma base, junto al cruce 1. El plano tiene `N` cruces, numerados del 1 al `N`, unidos por `M` calles de doble sentido, cada una con los segundos que tarda una ambulancia en recorrerla. El teléfono de emergencias suena toda la noche, y con cada llamada la operadora necesita un dato al instante: ¿cuántos segundos tardará la ambulancia de la base en llegar al cruce desde el que llaman?

Este es un **problema de programa**: lee el plano de calles y las llamadas de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `N` y `M`.
- Después, `M` líneas, cada una `a b w`: una calle entre los cruces `a` y `b` que se recorre en `w` segundos en cualquier sentido. Dos cruces pueden estar unidos por más de una calle.
- Después, una línea con `Q`, el número de llamadas.
- Por último, `Q` líneas, cada una con el cruce `t` de una llamada.

## Formato de salida

Escribe `Q` líneas, una por llamada y en orden: el menor tiempo en segundos del cruce 1 al cruce `t`, o `-1` si ninguna ruta por las calles llega a `t`. El cruce de la propia base está a 0 segundos.

### Ejemplo 1

Entrada:

```
4 4
1 2 5
2 3 2
1 3 9
3 4 1
3
3
4
2
```

Salida:

```
7
8
5
```

El cruce 3 está a 7 segundos pasando por el cruce 2 (5 + 2), más rápido que la calle directa de 9 segundos.

### Ejemplo 2

Entrada:

```
5 2
1 2 3
4 5 1
3
5
1
2
```

Salida:

```
-1
0
3
```

Los cruces 4 y 5 solo están unidos entre sí, así que la ambulancia no puede llegar a ellos.

### Ejemplo 3

Entrada:

```
3 3
2 1 10
1 2 4
2 3 6
2
3
3
```

Salida:

```
10
10
```

Dos calles unen los cruces 1 y 2; cuenta la más rápida. Puede haber varias llamadas desde el mismo cruce.

### Restricciones

- 1 ≤ N ≤ 10⁴
- 0 ≤ M ≤ 2·10⁴
- 1 ≤ a, b ≤ N, a ≠ b
- 1 ≤ w ≤ 10⁴
- 1 ≤ Q ≤ 10⁴
- 1 ≤ t ≤ N
