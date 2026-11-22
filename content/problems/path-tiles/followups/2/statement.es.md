El sendero es la misma franja, de 1 baldosa de ancho y `n` baldosas de largo, pavimentada
de un extremo a otro con baldosas cuadradas (1×1), losas cortas (1×2) y losas largas
(1×3). Dos pavimentos son distintos si las secuencias de piezas de un extremo al otro son
distintas.

Esta vez dos **losas largas nunca pueden tocarse**: una losa larga no puede ir seguida
directamente de otra losa larga. Cuenta los pavimentos que cumplen esta regla y devuelve
el número módulo 1 000 000 007.

### Ejemplo 1

```
n = 6
Salida: 23
```

De los 24 pavimentos sin la regla, solo larga-larga junta dos losas largas.

### Ejemplo 2

```
n = 5
Salida: 13
```

Dos losas largas necesitan 6 baldosas, así que con `n = 5` la regla no quita nada.

### Ejemplo 3

```
n = 4
Salida: 7
```

### Restricciones

- 1 ≤ n ≤ 10⁶
