Una casa del árbol se dibuja como un árbol binario, con la raíz arriba. Desde una ventana a la **derecha** del árbol se ve exactamente un nodo por nivel: el nodo más a la derecha de ese nivel (los demás quedan tapados detrás).

Dada `root`, devuelve los valores que se ven, del nivel de arriba hacia abajo.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [1, 2, 3, null, 5, null, 4]
Salida: [1, 3, 4]
```

### Ejemplo 2

```
root = [1, 2, 3, 4]
Salida: [1, 3, 4]
```

El nivel 3 solo tiene el nodo 4, a la izquierda, pero nada lo tapa.

### Ejemplo 3

```
root = []
Salida: []
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- -10⁴ ≤ valor de un nodo ≤ 10⁴
- El árbol tiene como mucho 1000 niveles.
