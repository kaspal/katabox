Tras un fin de semana fuera, unos amigos quieren repartir los gastos **a partes iguales**. `paid[i]` es lo que el amigo `i` gastó para el grupo, en euros.

Devuelve, para cada amigo, su saldo: cuánto pagó por encima de la parte que le toca. Un saldo positivo significa que los demás le deben dinero; uno negativo, que todavía tiene que pagar.

Se aceptan respuestas a menos de 10⁻⁶ del valor exacto.

### Ejemplo 1

```
paid = [30.0, 10.5, 20.25, 0.0]
Salida: [14.8125, -4.6875, 5.0625, -15.1875]
```

Entre todos gastaron 60.75, así que a cada uno le tocan 15.1875.

### Ejemplo 2

```
paid = [12.0, 12.0]
Salida: [0.0, 0.0]
```

### Ejemplo 3

```
paid = [8.0]
Salida: [0.0]
```

Quien viaja solo no le debe nada a nadie.

### Restricciones

- 1 ≤ paid.length ≤ 2·10⁴
- 0 ≤ paid[i] ≤ 10⁴
