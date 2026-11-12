Comprobar si cada número hasta `L` es primo, por separado en cada pregunta, repite el mismo trabajo miles de veces. ¿Puedes decidir de una vez qué números hasta el mayor límite son primos?
---
La criba de Eratóstenes: empieza con todos los números marcados como primos y, para cada primo `p`, tacha `p², p² + p, p² + 2p, …`. Basta con empezar en `p²`: los múltiplos menores tienen un factor primo más pequeño y ya están tachados.
---
Tras la criba, una cuenta acumulada `cuenta[x]` = número de primos ≤ `x` responde cada pregunta en O(1).
