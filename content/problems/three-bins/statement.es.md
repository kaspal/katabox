Una cinta de reciclaje lleva objetos de tres tipos: `0` es vidrio, `1` es papel y `2` es plástico. Antes de que los objetos lleguen a los contenedores, un brazo robótico debe reordenar la cinta para que primero vaya todo el vidrio, luego todo el papel y luego todo el plástico.

El brazo solo puede **intercambiar dos objetos** de la cinta, y en la cinta no cabe una segunda copia. Reordena `items` en el sitio, con una sola pasada por la cinta, y devuélvelo.

### Ejemplo 1

```
items = [2, 0, 1, 2, 0]
Salida: [0, 0, 1, 2, 2]
```

### Ejemplo 2

```
items = [1]
Salida: [1]
```

### Ejemplo 3

```
items = [2, 2, 0, 1, 1, 0]
Salida: [0, 0, 1, 1, 2, 2]
```

### Restricciones

- 1 ≤ items.length ≤ 10⁵
- items[i] es 0, 1 o 2

El juez solo comprueba el orden devuelto; la pasada única con intercambios es el ejercicio.
