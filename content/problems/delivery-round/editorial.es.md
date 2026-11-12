## Enfoque: programación dinámica sobre subconjuntos (Held–Karp)

El orden de las paradas ya visitadas no importa para el resto de la ruta: solo **cuáles** se han visitado y **dónde** está ahora la mensajera. Eso da el estado `(máscara, última)`, donde el bit `j` de `máscara` dice si la parada `j` ya se visitó (el bit 0, el almacén, siempre está activo) y `última` es la parada actual.

`mejor(máscara, última)` es el mínimo de minutos para visitar todas las paradas que no están en `máscara` y volver al almacén:

```
mejor(completa, última) = dist[última][0]
mejor(máscara, última) = mín sobre j fuera de máscara de dist[última][j] + mejor(máscara | (1 << j), j)
respuesta = mejor(1, 0)
```

Memorizar `mejor` (o llenar una tabla por máscaras en orden creciente) evalúa cada uno de los 2ⁿ · n estados una vez, cada uno en O(n).

**Por qué es correcto.** Cualquier ruta es el almacén, una primera parada `j` y una ruta por las paradas restantes que acaba en el almacén. Dado `(máscara, última)`, la forma más barata de terminar es la elección más barata de la siguiente parada más la forma más barata de terminar después, que es justo la recurrencia. Los trayectos siempre van directos, así que no hay que considerar otros caminos, aunque las distancias no cumplan la desigualdad triangular.

### Paso a paso

Ejemplo 1: `mejor({0,1,2}, 1) = dist[1][0] = 1` y `mejor({0,1,2}, 2) = 15`. Después `mejor({0,2}, 2) = dist[2][1] + 1 = 8`, `mejor({0,1}, 1) = dist[1][2] + 15 = 21`. Por último `mejor({0}, 0) = mín(2 + 21, 9 + 8) = 17`.

**Complejidad:** O(2ⁿ · n²) de tiempo y O(2ⁿ · n) de memoria: unos 6·10⁵ pasos con n = 12. Probar cada orden cuesta O(n!).
