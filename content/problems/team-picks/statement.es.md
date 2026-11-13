Un club deportivo pregunta a menudo: "¿de cuántas formas podemos elegir un equipo de `k` jugadores de una plantilla de `n`?" (El orden de elección no importa, así que es el coeficiente binomial C(n, k); elegir más jugadores de los que tiene la plantilla es imposible: 0 formas.)

Responde cada consulta `[n, k]` de `queries`. La respuesta puede ser enorme, así que devuélvela **módulo 1 000 003** (un primo): el resto de dividirla entre 1 000 003.

### Ejemplo 1

```
queries = [[5, 2], [6, 3], [4, 0]]
Salida: [10, 20, 1]
```

### Ejemplo 2

```
queries = [[3, 5]]
Salida: [0]
```

### Ejemplo 3

```
queries = [[40, 20]]
Salida: [115282]
```

C(40, 20) = 137 846 528 820, cuyo resto módulo 1 000 003 es 115 282.

### Restricciones

- 1 ≤ queries.length ≤ 10⁵
- 0 ≤ n, k ≤ 10⁵
