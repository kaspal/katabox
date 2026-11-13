Una biblioteca guarda los códigos de su catálogo en un árbol binario para que las búsquedas sean rápidas. El árbol es un **árbol de búsqueda** válido cuando, para cada nodo:

- todos los códigos de su subárbol izquierdo son **menores** que el código del nodo, y
- todos los de su subárbol derecho son **mayores**.

No se permiten códigos iguales. Dada `root`, devuelve si el árbol es un árbol de búsqueda válido. Un árbol vacío es válido.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [8, 3, 10, 1, 6, null, 14]
Salida: true
```

### Ejemplo 2

```
root = [8, 3, 10, 1, 9]
Salida: false
```

El 9 está en el subárbol izquierdo del 8 pero es mayor que 8, aunque sea mayor que su padre, el 3.

### Ejemplo 3

```
root = [5, 5]
Salida: false
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- -10⁹ ≤ valor de un nodo ≤ 10⁹
- El árbol tiene como mucho 1000 niveles.
