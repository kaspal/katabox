Un aserradero debe cortar un tronco de longitud `length` en cada posición de `marks` (medida desde el extremo izquierdo). Cada corte se hace sobre una pieza cada vez, y la sierra cobra la **longitud actual de la pieza** que se corta. Los cortes se pueden hacer en cualquier orden.

Devuelve el menor coste total para hacer todos los cortes.

### Ejemplo 1

```
length = 10, marks = [2, 4, 7]
Salida: 20
```

Cortar primero en 4 cuesta 10; después, 2 sobre la pieza 0–4 cuesta 4, y 7 sobre la pieza 4–10 cuesta 6: 20. Cortar en 2, luego en 4 y luego en 7 costaría 10 + 8 + 6 = 24.

### Ejemplo 2

```
length = 5, marks = [1]
Salida: 5
```

### Ejemplo 3

```
length = 12, marks = [3, 8, 10]
Salida: 24
```

### Restricciones

- 2 ≤ length ≤ 10⁶
- 1 ≤ marks.length ≤ 100
- 0 < marks[i] < length, todas distintas
