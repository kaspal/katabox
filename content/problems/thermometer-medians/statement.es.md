Una estación meteorológica registra las temperaturas de una en una, en décimas de grado, y tras cada lectura su pantalla muestra la **mediana** de todas las lecturas hasta ese momento. La mediana es el valor central de las lecturas ordenadas; con un número par de lecturas es la media de los dos valores centrales.

Dadas las lecturas `readings` en el orden en que llegan, devuelve la mediana que se muestra tras cada una.

Se aceptan respuestas a menos de 10⁻⁶ del valor exacto.

### Ejemplo 1

```
readings = [12, -4, 30, 7]
Salida: [12.0, 4.0, 12.0, 9.5]
```

Tras `12, -4` los valores centrales son −4 y 12, así que la mediana es 4. Tras las cuatro lecturas, ordenadas `−4, 7, 12, 30`, es (7 + 12) / 2 = 9.5.

### Ejemplo 2

```
readings = [5, 5, 5]
Salida: [5.0, 5.0, 5.0]
```

### Ejemplo 3

```
readings = [1, 2]
Salida: [1.0, 1.5]
```

### Restricciones

- 1 ≤ readings.length ≤ 10⁵
- −10⁴ ≤ readings[i] ≤ 10⁴
