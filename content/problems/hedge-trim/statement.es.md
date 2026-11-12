Un seto de jardín crece como un árbol binario de ramas. Cada rama tiene una puntuación de vigor `val`, negativa si la rama está enferma. El jardinero poda el seto desde las puntas hacia dentro:

- Una rama solo se mira después de haber atendido sus dos subramas.
- Si el vigor total de la rama junto con **lo que queda** debajo de ella es negativo, la rama se corta con todo lo que tiene debajo.
- Un total de cero o más se conserva.

Devuelve el seto después de todos los cortes. Si se corta la propia raíz, el seto queda vacío.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [3, -5, 2, 1, -4]
Salida: [3, null, 2]
```

La rama -4 se corta. Lo que queda de -5 suma -5 + 1 = -4, así que también se corta, con el 1 que tiene debajo. La raíz conserva 3 + 2 = 5.

### Ejemplo 2

```
root = [-1, 1]
Salida: [-1, 1]
```

-1 + 1 = 0 no es negativo, así que no se corta nada.

### Ejemplo 3

```
root = [1, -3, null, 2]
Salida: [1]
```

### Ejemplo 4

```
root = [-1]
Salida: []
```

### Restricciones

- 0 ≤ número de nodos ≤ 10⁴
- -10⁴ ≤ valor de un nodo ≤ 10⁴
- El árbol tiene como mucho 1000 niveles.
