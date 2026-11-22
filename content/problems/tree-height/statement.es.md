Lucía dibuja árboles genealógicos para un club de genealogía y quiere saber cuántas generaciones abarca cada uno, para saber qué altura darle al imprimirlo. Su función da un número para cada árbol, pero los dibujos impresos siempre salen con una fila vacía de más.

Dada la raíz `root` de un árbol binario, devuelve su altura: el número de nodos del camino más largo desde la raíz hasta una hoja. El árbol vacío tiene altura 0.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se listan y los `null` del final se omiten. `[]` es el árbol vacío.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
root = [5, 3, 8, null, 4]
Salida: 3
```

El camino más largo es 5 → 3 → 4.

### Ejemplo 2

```
root = [26, 94, 15, 20]
Salida: 3
```

### Ejemplo 3

```
root = [-8, -57, 88]
Salida: 2
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- -100 ≤ valor de un nodo ≤ 100
- El árbol tiene como mucho 1000 niveles.
