Un mapa del tesoro es un árbol binario de cuevas; cada cueva tiene `val` monedas, o una deuda cuando `val` es negativo. Una **ruta** visita una secuencia de cuevas unidas como padre–hijo, sin visitar ninguna dos veces, y puede empezar y terminar en cualquier cueva (no tiene por qué incluir la raíz). Una ruta tiene al menos una cueva.

Devuelve el mayor total de monedas a lo largo de alguna ruta.

Los árboles se escriben por niveles, de arriba abajo y de izquierda a derecha, con `null` para un hijo que falta; los hijos de un nodo que falta no se escriben, y los null del final se omiten. `[]` es el árbol vacío.

### Ejemplo 1

```
root = [2, -1, 3]
Salida: 5
```

La ruta 2 → 3.

### Ejemplo 2

```
root = [-5, 4, 8, null, null, -2, 6]
Salida: 14
```

La ruta −2 → 8 → 6 suma 12; 8 → 6 suma 14; subir hasta el −5 solo restaría monedas.

### Ejemplo 3

```
root = [-3]
Salida: -3
```

### Restricciones

- 1 ≤ número de nodos ≤ 10⁴
- -1000 ≤ valor de un nodo ≤ 1000
- El árbol tiene como mucho 1000 niveles.
