El sendero es la misma franja, de 1 baldosa de ancho y `n` baldosas de largo, pavimentada
de un extremo a otro con baldosas cuadradas (1×1), losas cortas (1×2) y losas largas
(1×3), tantas de cada como haga falta. Dos pavimentos son distintos si las secuencias de
piezas de un extremo al otro son distintas.

Esta vez el sendero es **larguísimo**: `n` puede llegar a 10¹⁵ (cabe en un entero de 64
bits y en un número de JavaScript). Devuelve el número de pavimentos módulo
**1 000 003**.

### Ejemplo 1

```
n = 3
Salida: 4
```

Cuadrada-cuadrada-cuadrada, cuadrada-corta, corta-cuadrada o una losa larga.

### Ejemplo 2

```
n = 5
Salida: 13
```

### Ejemplo 3

```
n = 4
Salida: 7
```

### Restricciones

- 1 ≤ n ≤ 10¹⁵
