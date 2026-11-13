Una furgoneta de reparto debe recorrer `target` kilómetros por una carretera recta. Sale con `fuel` litros y gasta exactamente un litro por kilómetro; el depósito admite cualquier cantidad. A lo largo de la carretera hay gasolineras, `stations[i] = [position, litres]`: parar en una añade todos sus `litres` al depósito.

Devuelve el menor número de paradas necesario para llegar al destino, o `-1` si no se puede llegar. Llegar a una gasolinera o al destino con el depósito vacío cuenta como llegar.

### Ejemplo 1

```
target = 50, fuel = 15, stations = [[10, 20], [20, 5], [30, 30], [40, 10]]
Salida: 2
```

Para en el 10 (quedan 5 litros + 20 = 25, suficiente para llegar al 30), para en el 30 a por 30 más y termina. Una parada no basta: con 15 litros solo está a su alcance la gasolinera del 10, y con ella llega al 35.

### Ejemplo 2

```
target = 30, fuel = 30, stations = [[10, 5]]
Salida: 0
```

### Ejemplo 3

```
target = 40, fuel = 10, stations = [[10, 10], [25, 30]]
Salida: -1
```

Tras parar en el 10, la furgoneta tiene 10 litros, que no bastan para los 15 km hasta la siguiente gasolinera.

### Restricciones

- 1 ≤ target, fuel ≤ 10⁹
- 0 ≤ stations.length ≤ 10⁵
- 0 < position < target, posiciones estrictamente crecientes
- 1 ≤ litres ≤ 10⁹
