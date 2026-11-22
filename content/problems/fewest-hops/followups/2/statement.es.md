La red de radio tiene `n` estaciones numeradas del `0` al `n − 1`, pero ahora cada enlace
es **de un solo sentido**: una pareja `[a, b]` de `links` permite que la estación `a`
envíe a la estación `b`, no al revés. Pasar el mensaje por un enlace es un **salto**.

Antes de enviar, puedes **dar la vuelta a como mucho un enlace**: un enlace `[a, b]` al
que das la vuelta funciona entonces solo de `b` a `a`, y ya no de `a` a `b`. Todos los
demás enlaces mantienen su sentido.

Devuelve el menor número de saltos para llevar un mensaje de la estación `0` a la
estación `n − 1`, eligiendo del mejor modo qué enlace girar (o ninguno), o `-1` si ninguna
elección lo hace posible. Para `n = 1` la respuesta es `0`: el mensaje empieza donde tiene
que llegar.

### Ejemplo 1

```
n = 4, links = [[0, 1], [2, 1], [2, 3]]
Salida: 3
```

Da la vuelta a `[2, 1]` para que funcione de 1 a 2; entonces 0 → 1 → 2 → 3 cuesta tres
saltos.

### Ejemplo 2

```
n = 4, links = [[2, 0], [3, 1], [3, 2]]
Salida: -1
```

Girar `[2, 0]` permite llegar a la estación 2, pero ningún enlace sale de la estación 2
hacia la estación 3.

### Ejemplo 3

```
n = 4, links = [[0, 1], [1, 2], [2, 1]]
Salida: -1
```

### Restricciones

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` con 0 ≤ a, b < n y a ≠ b; el mismo enlace puede aparecer más de una
  vez
