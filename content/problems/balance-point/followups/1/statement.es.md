La tabla del problema base (la caja `i` empuja con fuerza `weights[i]`; una fuerza
negativa es un globo de helio que tira hacia arriba) ahora descansa sobre **dos**
apoyos, bajo dos cajas distintas `i < j`. Los apoyos dividen la tabla en tres tramos:
las cajas a la izquierda de `i`, las cajas estrictamente entre `i` y `j`, y las cajas a
la derecha de `j`.

Un par de apoyos equilibra la tabla cuando los tres tramos tienen la misma fuerza total.
Las cajas de los apoyos no cuentan para ningún tramo, y un tramo vacío suma 0 (así que
los apoyos pueden estar juntos o en los extremos). Devuelve **cuántos** pares `i < j`
equilibran la tabla.

### Ejemplo 1

```
weights = [2, 5, 2, 9, 1, 1]
Salida: 1
```

Apoyos en 1 y 3: los tramos son [2], [2] y [1, 1], y cada uno suma 2.

### Ejemplo 2

```
weights = [2, -2, 0, -2, 1]
Salida: 1
```

Apoyos en 3 y 4: el tramo izquierdo suma 2 + (−2) + 0 = 0 y los otros dos están vacíos.

### Ejemplo 3

```
weights = [-2, -2, 0, -1]
Salida: 0
```

### Restricciones

- 2 ≤ weights.length ≤ 10⁵
- −10⁴ ≤ weights[i] ≤ 10⁴
- La respuesta puede pasar de 32 bits (una tabla de ceros se equilibra con cualquier
  par); cabe en un entero de 64 bits y en un número de JavaScript.
