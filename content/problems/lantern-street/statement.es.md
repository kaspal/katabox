Los callejones de un pueblo se ramifican desde la plaza como un árbol binario: cada nodo es una farola, y su valor es lo que brillaría su farolillo. Para la fiesta se encienden farolillos, pero dos farolas unidas directamente por un callejón (un padre y su hijo) no pueden estar **las dos** encendidas, o la instalación se sobrecarga.

Devuelve el mayor brillo total que se puede encender.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [4, 1, 6, 2, 3, null, 5]
Salida: 14
```

Enciende la raíz (4) y las farolas 2, 3 y 5, dos niveles más abajo.

### Ejemplo 2

```
root = [2, 9, 1, null, null, 7, 8]
Salida: 24
```

Enciende el 9, el 7 y el 8.

### Ejemplo 3

```
root = []
Salida: 0
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- 0 ≤ valor de un nodo ≤ 10⁴
- El árbol tiene como mucho 1000 niveles.
