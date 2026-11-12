Un club de tenis de mesa apunta la temporada de su equipo como una cadena: `W` para una victoria, `L` para una derrota y `D` para un empate, una letra por partido.

El entrenador llama *equilibrada* a una racha de partidos **consecutivos** que tiene exactamente tantas victorias como derrotas (los empates no cuentan). ¿Cuántas rachas equilibradas tiene la temporada? Una racha es cualquier tramo no vacío de partidos consecutivos; las rachas en posiciones distintas cuentan por separado, aunque se lean igual.

### Ejemplo 1

```
results = "WDL"
Salida: 2
```

Las rachas equilibradas son "D" (ninguna victoria, ninguna derrota) y "WDL".

### Ejemplo 2

```
results = "WWLL"
Salida: 2
```

"WL" en el centro y la temporada entera "WWLL".

### Ejemplo 3

```
results = "WLW"
Salida: 2
```

"WL" y "LW".

### Restricciones

- 1 ≤ results.length ≤ 10⁵
- `results` solo contiene `W`, `L` y `D`
