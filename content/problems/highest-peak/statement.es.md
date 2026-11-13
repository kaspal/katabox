Una aplicación de senderismo guarda el perfil de una ruta de montaña como la altitud en cada hito, en `heights`. La ruta siempre sube estrictamente y después baja estrictamente: las altitudes crecen estrictamente hasta la cumbre y decrecen estrictamente después (la subida o la bajada pueden estar vacías, si la cumbre está en un extremo).

Devuelve el índice de la cumbre, el hito más alto.

### Ejemplo 1

```
heights = [1, 4, 9, 7, 2]
Salida: 2
```

### Ejemplo 2

```
heights = [5, 3, 1]
Salida: 0
```

La ruta empieza en la cumbre.

### Ejemplo 3

```
heights = [2, 6]
Salida: 1
```

### Restricciones

- 1 ≤ heights.length ≤ 10⁵
- 0 ≤ heights[i] ≤ 10⁹
- heights crece estrictamente y después decrece estrictamente.

Un recorrido O(n) también pasa las pruebas; el ejercicio es la búsqueda O(log n).
