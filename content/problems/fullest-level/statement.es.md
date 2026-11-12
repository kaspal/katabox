Un guarda de huerto dibuja un frutal como un árbol binario: cada nodo es una horquilla de ramas, y su valor es el número de manzanas que cuelgan de ella. La raíz está en el **nivel 1**, sus hijos en el nivel 2, y así sucesivamente.

Devuelve el nivel cuyas horquillas tienen más manzanas en total. Si varios niveles empatan, devuelve el más cercano a la raíz. Para un árbol vacío, devuelve `0`.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. Así, `[4, 7, 2, null, 1]` es 4 con los hijos 7 y 2, donde 7 solo tiene un hijo derecho, el 1. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [4, 7, 2, null, 1, 8, 3]
Salida: 3
```

El nivel 1 tiene 4 manzanas, el nivel 2 tiene 7 + 2 = 9, y el nivel 3 tiene 1 + 8 + 3 = 12.

### Ejemplo 2

```
root = [6, 3, 3]
Salida: 1
```

Los niveles 1 y 2 tienen 6 manzanas cada uno; el nivel 1 está más cerca de la raíz.

### Ejemplo 3

```
root = []
Salida: 0
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- 0 ≤ valor de un nodo ≤ 10⁴
- El árbol tiene como mucho 1000 niveles.
