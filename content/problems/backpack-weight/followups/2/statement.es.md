El problema de la excursionista es el mismo: el objeto `i` pesa `weights[i]` y vale
`values[i]`, cada objeto se lleva una vez o se deja, y el peso total no puede pasar de
`capacity`. Devuelve el mayor valor total de objetos que caben.

Lo que cambia es la escala. La mochila es **enorme** (`capacity` hasta 10⁹ y pesos hasta
10⁷), pero cada objeto vale como mucho 100.

### Ejemplo 1

```
weights = [3000000, 4000000, 2000000, 5000000], values = [30, 50, 15, 60], capacity = 7000000
Salida: 80
```

Los objetos 0 y 1 pesan 7 000 000 y valen 80.

### Ejemplo 2

```
weights = [140892, 596854, 888599, 841236], values = [97, 8, 32, 15], capacity = 2078005
Salida: 144
```

Los objetos 0, 2 y 3 pesan 1 870 727 y valen 144; los cuatro juntos pesarían demasiado.

### Ejemplo 3

```
weights = [905036, 993870, 890299, 59299], values = [11, 10, 46, 21], capacity = 2809052
Salida: 78
```

### Restricciones

- 1 ≤ weights.length = values.length ≤ 100
- 1 ≤ weights[i] ≤ 10⁷
- 0 ≤ values[i] ≤ 100
- 0 ≤ capacity ≤ 10⁹
