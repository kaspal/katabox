Al terminar unos juegos juveniles, la organización imprime el medallero. `countries[i]` ganó `medals[i] = [gold, silver, bronze]` (oros, platas y bronces).

Los países se ordenan por oros (más primero); los empates se deshacen por platas y después por bronces (más primero). Los países con exactamente las mismas medallas se listan por nombre en orden alfabético.

Devuelve los nombres de los países en el orden del medallero.

### Ejemplo 1

```
countries = ["norway", "chile", "kenya", "japan"],
medals = [[2, 1, 0], [2, 1, 3], [0, 5, 5], [2, 3, 0]]
Salida: ["japan", "chile", "norway", "kenya"]
```

Tres países tienen 2 oros; japan tiene más platas; chile supera a norway en bronces. Las 10 medallas de kenya no incluyen ningún oro.

### Ejemplo 2

```
countries = ["peru", "fiji"], medals = [[1, 0, 0], [1, 0, 0]]
Salida: ["fiji", "peru"]
```

### Ejemplo 3

```
countries = ["malta"], medals = [[0, 0, 0]]
Salida: ["malta"]
```

### Restricciones

- 1 ≤ countries.length = medals.length ≤ 10⁴
- Los nombres tienen de 1 a 20 letras minúsculas del alfabeto inglés y son todos distintos.
- 0 ≤ gold, silver, bronze ≤ 1000
