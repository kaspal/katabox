Una asociación de historia familiar ha dibujado un árbol genealógico con `n` personas, numeradas del `0` al `n − 1`. Para cada persona, `parent[i]` es el número de la persona de la que desciende directamente, o `-1` para la única fundadora en lo alto. Todo el mundo desciende de la fundadora.

La fundadora es la primera generación, sus hijos la segunda, y así sucesivamente. ¿Cuántas **generaciones** tiene el árbol?

### Ejemplo 1

```
parent = [-1, 0, 0, 1, 1, 4]
Salida: 4
```

Generación 1: la persona 0. Generación 2: las personas 1 y 2. Generación 3: las personas 3 y 4. Generación 4: la persona 5.

### Ejemplo 2

```
parent = [-1]
Salida: 1
```

### Ejemplo 3

```
parent = [2, -1, 1]
Salida: 3
```

La fundadora es la persona 1, su hija es la persona 2 y la persona 0 es hija de la 2. Los números no siguen las generaciones.

### Restricciones

- 1 ≤ n ≤ 10⁵
- Exactamente un `parent[i]` es `-1`; cada uno de los demás está entre `0` y `n − 1`
- Siguiendo `parent` desde cualquiera siempre se llega a la fundadora (no hay bucles)
