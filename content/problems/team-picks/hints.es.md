C(n, k) = n! / (k! · (n − k)!). Calcularlo desde cero en cada consulta es O(k) por consulta. ¿Qué puedes preparar una sola vez para todas?
---
Precalcula los factoriales 0!, 1!, …, (n máx)! módulo p. El problema es la división: módulo un primo p, dividir entre `x` es multiplicar por su **inverso**, `x^(p−2) mod p` (pequeño teorema de Fermat).
---
Precalcula también los inversos de los factoriales: calcula `inv[N] = (N!)^(p−2)` con potencia rápida y después `inv[i − 1] = inv[i] · i` hacia abajo. Cada consulta es entonces `fact[n] · inv[k] · inv[n − k] mod p`.
