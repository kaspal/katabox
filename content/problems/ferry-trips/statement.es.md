Unos excursionistas esperan en un lago una pequeña barca de remos. La barca lleva **como mucho dos** excursionistas por viaje, y su peso total no puede superar `limit`. Nadie pesa más de `limit`, así que cualquiera puede cruzar solo si hace falta.

Dados los pesos `weights` de los excursionistas, devuelve el menor número de viajes necesario para cruzarlos a todos. (Cuenta los viajes de ida con pasajeros; la vuelta de la barca es gratis).

### Ejemplo 1

```
weights = [60, 45, 80, 35], limit = 100
Salida: 3
```

Un plan: 60 + 35, después 45 solo, después 80 solo. El de 80 no puede compartir con nadie y entre los demás solo cabe una pareja.

### Ejemplo 2

```
weights = [50, 50, 50, 50], limit = 100
Salida: 2
```

### Ejemplo 3

```
weights = [70, 90, 99], limit = 100
Salida: 3
```

### Restricciones

- 1 ≤ weights.length ≤ 10⁵
- 1 ≤ weights[i] ≤ limit ≤ 3·10⁴
