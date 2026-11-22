Ingrid anota la altitud de cada control de sus carreras de montaña y le gusta saber cuál es la subida más larga que podría escoger en una ruta. Encontró un truco rápido en internet y lo programó, pero en algunas rutas su función presume de subidas que nunca existieron.

`heights` lista las altitudes de los controles en el orden en que los pasa. Escoge algunos controles, manteniendo su orden, de forma que cada control escogido sea **estrictamente más alto** que el anterior escogido. Devuelve el mayor número de controles que puede tener una elección así.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
heights = [3, 1, 3, 3, 4, 2, 5]
Salida: 4
```

Por ejemplo 1, 3, 4, 5. Dos controles a la misma altura no pueden escogerse los dos.

### Ejemplo 2

```
heights = [5,1,3,1,4]
Salida: 3
```

### Ejemplo 3

```
heights = [1,1,3,2,3]
Salida: 3
```

### Restricciones

- 1 ≤ heights.length ≤ 10⁵
- 1 ≤ heights[i] ≤ 10⁹
