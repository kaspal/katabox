Los caminos de un jardín botánico forman un árbol binario: cada nodo es una fuente, y cada enlace padre–hijo es un camino. Un paseo puede empezar y terminar en dos fuentes cualesquiera y nunca usa un camino dos veces; no tiene por qué pasar por la raíz.

Devuelve el número de caminos del paseo más largo posible. Un jardín con cero o una fuente no tiene paseo: devuelve `0`.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [1, 2, 3, 4, 5]
Salida: 3
```

Por ejemplo 4 → 2 → 1 → 3.

### Ejemplo 2

```
root = [1, 2, null, 3, 4, 5, null, null, 6]
Salida: 4
```

El paseo 5 → 3 → 2 → 4 → 6 no toca la raíz.

### Ejemplo 3

```
root = [9]
Salida: 0
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- -10⁴ ≤ valor de un nodo ≤ 10⁴
- El árbol tiene como mucho 1000 niveles.
