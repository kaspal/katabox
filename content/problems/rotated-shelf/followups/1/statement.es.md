La estantería es la misma del problema base: carpetas con números de catálogo distintos,
ordenadas de menor a mayor, después de que un ayudante pasara un bloque del principio al
final. Los números de catálogo son positivos.

Las carpetas nuevas necesitan número, así que la bibliotecaria pregunta por los **números
libres**: un número positivo está libre cuando ninguna carpeta de la estantería lo tiene.
Para cada `k` de `queries`, devuelve el `k`-ésimo número libre más pequeño (`k = 1` es el
menor), en el orden de las preguntas.

### Ejemplo 1

```
shelf = [9, 12, 2, 4, 5], queries = [1, 3, 4, 8]
Salida: [1, 6, 7, 13]
```

Los números libres son 1, 3, 6, 7, 8, 10, 11, 13, 14, …

### Ejemplo 2

```
shelf = [1, 2, 3], queries = [1, 2]
Salida: [4, 5]
```

### Ejemplo 3

```
shelf = [7], queries = [6, 7]
Salida: [6, 8]
```

### Restricciones

- 1 ≤ shelf.length ≤ 10⁵
- 1 ≤ shelf[i] ≤ 10⁹; los números son todos distintos, y `shelf` es una lista ordenada
  con un bloque pasado del principio al final.
- 1 ≤ queries.length ≤ 10⁵
- 1 ≤ queries[j] ≤ 10⁹ (toda respuesta queda por debajo de 1,1·10⁹)
