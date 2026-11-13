## Enfoque: combinar las congruencias una a una (teorema chino del resto generalizado)

La línea `(r, m)` sale en el minuto `t` exactamente cuando `t ≡ r (mod m)` (con `0 ≤ r < m`, todo `t ≥ 0` con ese resto es una de sus salidas). Así que un horario pide el menor `t ≥ 0` que cumple todas sus congruencias a la vez.

**Por qué buscar es demasiado lento.** El patrón se repite tras el mcm de los periodos, así que comprobar cada minuto por debajo de él es correcto, pero el mcm puede llegar a 9·10¹⁵. Una búsqueda más lista recorre solo las salidas de la línea con el periodo más largo, `r, r + m, r + 2m, …`; aun así son hasta mcm / m pasos, y con periodos de como mucho 10⁶ y un mcm de 10¹² o más eso es al menos un millón de pasos, a menudo miles de millones, para un solo horario. En su lugar, se pliegan las congruencias en una sola.

**Combinar dos congruencias.** Supón que todas las líneas vistas salen en `t ≡ x (mod M)`, con `0 ≤ x < M`. Esos minutos son `t = x + M·k`. La siguiente línea exige

```
x + M·k ≡ r (mod m)   ⇔   M·k ≡ r − x (mod m)
```

Sea `g = mcd(M, m)`. El lado izquierdo es múltiplo de `g`, y `m` también, así que hace falta `g | (r − x)`; si no, las líneas nunca coinciden: escribe `-1`. Si se cumple, divide todo entre `g`: con `m' = m / g`, `M' = M / g` y `d = (r − x) / g`,

```
M'·k ≡ d (mod m'),   mcd(M', m') = 1   ⇒   k ≡ d · inverso(M', m') (mod m')
```

Toma ese `k` en `[0, m')`. Después `x ← x + M·k` y `M ← M·m'`, que es `mcm(M, m)`. Empieza en `(x, M) = (0, 1)` (todos los minutos) y combina cada línea por turno; la `x` final es la respuesta.

**El inverso** sale del algoritmo de Euclides extendido: ejecutar Euclides sobre `(a, m')` llevando el coeficiente `s` con `a·s ≡ resto (mod m')` termina con resto 1 y `a·s ≡ 1`. Cuando `m' = 1` el inverso es 0 y `k = 0`.

**Por qué el resultado es el menor minuto:** las soluciones del sistema combinado son exactamente `x (mod mcm)`, y tras cada paso `0 ≤ x ≤ (M − 1) + M·(m' − 1) = M·m' − 1`, así que `x` es la menor no negativa.

**Exactitud.** Todos los valores caben en un entero de 64 bits, e incluso en un número de TypeScript, exacto hasta 2⁵³ ≈ 9,007·10¹⁵:

- `M`, `x` y cada mcm parcial son como mucho el mcm final ≤ 9·10¹⁵; `r − x`, el mcd y la división exacta `(r − x) / g` tampoco pasan de ese tamaño;
- `d mod m'` y el inverso son ambos menores que `m' ≤ 10⁶`, así que su producto es menor que 10¹²;
- `M·k < M·m' = mcm ≤ 9·10¹⁵`, y la nueva `x` también.

Así que no hace falta aritmética de 128 bits ni trucos de multiplicación modular, siempre que solo se multipliquen estos pares (nunca `M` por un inverso, por ejemplo). Reduce `d` a `[0, m')` primero, porque `r − x` puede ser negativo (`((d % m') + m') % m'` en TypeScript y PHP, `rem_euclid` en Rust; el `%` de Python ya es no negativo).

**Complejidad:** O(n·log m) por horario, O(T·n·log m) en total.

**Entrada:** lee todos los tokens; cada horario es `n` seguido de `2n` números. Aunque ya se sepa que la respuesta es `-1`, sigue leyendo las líneas que le quedan a ese horario.
