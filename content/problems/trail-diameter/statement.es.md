Un parque natural está preparando su caminata de resistencia anual. Los cruces del parque están numerados del 1 al `N`, y `N − 1` senderos de doble sentido los unen de modo que entre dos cruces cualesquiera hay **exactamente una ruta** (los senderos nunca forman un circuito). Los organizadores quieren que la caminata siga la ruta más larga de este tipo, así que preguntan: ¿cuál es el mayor número de senderos en la ruta entre dos cruces?

Este es un **problema de programa**: lee los mapas de los parques de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `T`, el número de parques.
- Cada parque empieza con una línea que contiene `N`, el número de cruces, seguida de `N − 1` líneas `a b`: un sendero entre los cruces `a` y `b`. Un parque con un solo cruce no tiene líneas de senderos.

## Formato de salida

Escribe `T` líneas, una por parque y en orden: el número de senderos de la ruta más larga entre dos de sus cruces.

### Ejemplo 1

Entrada:

```
2
5
1 2
2 3
3 4
2 5
1
```

Salida:

```
3
0
```

En el primer parque, las rutas 1-2-3-4 y 5-2-3-4 usan 3 senderos cada una. El segundo parque es un solo cruce.

### Ejemplo 2

Entrada:

```
1
6
4 1
4 2
4 3
4 5
4 6
```

Salida:

```
2
```

Todos los senderos llegan al cruce 4, así que la ruta más larga va de un extremo, pasando por el 4, a otro extremo.

### Ejemplo 3

Entrada:

```
1
7
1 2
1 3
2 4
2 5
3 6
6 7
```

Salida:

```
5
```

La ruta más larga es 4-2-1-3-6-7 (o 5-2-1-3-6-7).

### Restricciones

- 1 ≤ T ≤ 1000
- 1 ≤ N ≤ 20 000, y la suma de los `N` de todos los parques es como mucho 20 000.
- 1 ≤ a, b ≤ N y a ≠ b; los senderos de cada parque unen todos sus cruces sin formar circuitos.
