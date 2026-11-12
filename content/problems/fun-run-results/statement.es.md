Tras la carrera solidaria del pueblo, la organización imprime el tablón de resultados. El corredor `i` se llama `names[i]` y llegó a meta en `times[i]` segundos.

Devuelve los nombres en el orden del tablón: primero el **más rápido**. Los corredores con el mismo tiempo aparecen en **orden alfabético** de sus nombres (un nombre que es el comienzo de otro va antes, así que "tom" va antes que "tomas").

### Ejemplo 1

```
names = ["ana", "leo", "bea", "zoe"], times = [1810, 1795, 1810, 2000]
Salida: ["leo", "ana", "bea", "zoe"]
```

Leo fue el más rápido. Ana y Bea empataron con 1810 segundos, así que aparecen en orden alfabético.

### Ejemplo 2

```
names = ["kim"], times = [3600]
Salida: ["kim"]
```

### Ejemplo 3

```
names = ["tom", "al", "tomas"], times = [500, 500, 500]
Salida: ["al", "tom", "tomas"]
```

### Restricciones

- 1 ≤ names.length ≤ 10⁴, times.length = names.length
- 1 ≤ times[i] ≤ 10⁵
- Los nombres tienen de 1 a 10 letras minúsculas del alfabeto inglés y son todos distintos
