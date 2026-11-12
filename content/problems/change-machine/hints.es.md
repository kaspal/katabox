Coger siempre la ficha más grande que cabe puede fallar (mira el Ejemplo 1). Probar todas las combinaciones es correcto, pero se dispara.
---
La última ficha pagada tiene algún valor `c`. Entonces el resto es la mejor forma de pagar `amount − c`. Las mismas cantidades pequeñas aparecen una y otra vez.
---
Rellena `best[x]` para `x = 0 … amount`: `best[0] = 0` y `best[x] = 1 + min(best[x − c])` sobre las fichas `c ≤ x` cuyo `best[x − c]` sea alcanzable. La respuesta es `best[amount]`, o `-1` si nunca se alcanzó.
