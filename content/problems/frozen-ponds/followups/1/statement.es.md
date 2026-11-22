Esta vez la guarda quiere vallar cada estanque. Los estanques se forman como antes: las celdas heladas `#` que se tocan por un lado son del mismo estanque. La **longitud de valla** de un estanque es el número de lados de sus celdas que no tocan otra celda helada: lados que dan a agua o tierra (`.`) o que están en el borde del mapa. Dos celdas del mismo estanque que se tocan no llevan valla entre ellas.

Devuelve la longitud de valla de cada estanque, de **mayor a menor** (las longitudes iguales aparecen simplemente una tras otra). Devuelve una lista vacía si no hay nada helado.

### Ejemplo 1

```
ice = ["##..#", "#...#", "..#..", ".##.."]
Salida: [8, 8, 6]
```

Los dos estanques en forma de L, de 3 celdas, necesitan 8 lados cada uno, y el estanque de 2 celdas del borde derecho necesita 6.

### Ejemplo 2

```
ice = ["..##.", "..#..", "....#", "..#.#"]
Salida: [8, 6, 4]
```

### Ejemplo 3

```
ice = ["#.#..", "###..", ".....", "....."]
Salida: [12]
```

Un solo estanque de 5 celdas: 20 lados, menos 2 por cada uno de los 4 sitios donde se tocan dos de sus celdas.

### Restricciones

- 1 ≤ ice.length ≤ 400
- 1 ≤ ice[i].length ≤ 400, todas las filas de la misma longitud
- Cada carácter es `#` o `.`
