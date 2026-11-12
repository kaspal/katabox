Probar cada orden de las otras 11 paradas son 11! ≈ 4·10⁷ rutas, cada una sumada sobre 12 trayectos. Muchos órdenes comparten el comienzo: cuando la mensajera ha visitado un **conjunto** de paradas y está en una parada concreta, el orden que siguió para llegar ya no importa para el resto.
---
Que el estado sea `(conjunto visitado, parada actual)`. La forma más barata de terminar desde ese estado (visitar las paradas que faltan y volver al 0) solo depende del estado. Hay 2ⁿ · n estados.
---
Codifica el conjunto visitado como una máscara de bits. `mejor(máscara, última)` = `dist[última][0]` cuando ya se visitaron todas, y si no, el mínimo sobre las `j` no visitadas de `dist[última][j] + mejor(máscara | 1 << j, j)`. Memorízala; la respuesta es `mejor(1, 0)`.
