Los códigos del catálogo de la biblioteca están guardados en un árbol de búsqueda válido: para cada nodo, los códigos de su subárbol izquierdo son menores y los de su subárbol derecho son mayores, y todos los códigos son distintos.

Dados `root` y `k`, devuelve el `k`-ésimo código más pequeño (`k = 1` es el menor).

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [8, 3, 10, 1, 6, null, 14], k = 3
Salida: 6
```

Los códigos en orden son 1, 3, 6, 8, 10, 14.

### Ejemplo 2

```
root = [8, 3, 10, 1, 6, null, 14], k = 6
Salida: 14
```

### Ejemplo 3

```
root = [5], k = 1
Salida: 5
```

### Restricciones

- 1 ≤ número de nodos ≤ 10⁴, y el árbol es un árbol de búsqueda válido.
- -10⁹ ≤ valor de un nodo ≤ 10⁹
- 1 ≤ k ≤ número de nodos
- El árbol tiene como mucho 1000 niveles.
