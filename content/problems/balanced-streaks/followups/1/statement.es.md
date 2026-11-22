El club sigue apuntando la temporada como una cadena de `W` (victoria), `L` (derrota) y
`D` (empate), y una racha de partidos **consecutivos** sigue siendo *equilibrada* cuando
tiene exactamente tantas victorias como derrotas (los empates no cuentan). Esta vez el
entrenador no quiere contar las rachas equilibradas: devuelve la **longitud de la más
larga**.

Una racha formada solo por empates es equilibrada. Si ninguna racha es equilibrada (por
ejemplo `"WW"`), devuelve `0`.

### Ejemplo 1

```
results = "WWDLWLLW"
Salida: 7
```

La racha "WDLWLLW" (todos los partidos menos el primero) tiene tres victorias y tres derrotas.

### Ejemplo 2

```
results = "WLLLD"
Salida: 2
```

### Ejemplo 3

```
results = "WDDD"
Salida: 3
```

"DDD" no tiene victorias ni derrotas.

### Restricciones

- 1 ≤ results.length ≤ 10⁵
- `results` solo contiene `W`, `L` y `D`
