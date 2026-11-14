## La idea

El **máximo común divisor** de dos números enteros positivos, `gcd(a, b)`, es el mayor número que divide a ambos. Probar cada candidato desde `min(a, b)` hacia abajo funciona, pero puede costar cientos de intentos. El algoritmo de Euclides solo necesita unas pocas divisiones.

Se apoya en un único hecho: **`a` y `b` tienen exactamente los mismos divisores comunes que `b` y `a mod b`**. Escribe `a = q · b + r`. Todo lo que divide a `a` y a `b` también divide a `r = a - q · b`; todo lo que divide a `b` y a `r` también divide a `a = q · b + r`. Así que cambiar el par `(a, b)` por `(b, a mod b)` nunca cambia la respuesta, y los números no dejan de encoger. Cuando `b` llega a 0, el par es `(g, 0)`, y el mayor divisor de `g` y de 0 es el propio `g`.

El ejemplo repite ese único cambio mientras `b` no sea 0 y cuenta las divisiones. La salida es `[mcd, número de divisiones]`. Si `a` es menor que `b`, la primera división solo los intercambia, porque `a mod b = a`.

Una vez conocido el mcd, sale el **mínimo común múltiplo**: `lcm(a, b) = a · b / gcd(a, b)`. Divide antes de multiplicar, `a / gcd · b`, para que el valor intermedio siga siendo pequeño.

### La entrada por defecto

`a = 336`, `b = 252` (el ajuste "un gran factor común"). Primera división: `336 mod 252 = 84`, así que el par pasa a ser `(252, 84)`. Segunda: `252 mod 84 = 0`, así que pasa a ser `(84, 0)` y el bucle se detiene. La salida es `[84, 2]`: dos divisiones. Los dos números son múltiplos de 84 (`336 = 4 · 84`, `252 = 3 · 84`), y su mcm es `336 / 84 · 252 = 1008`.

## Pasos

1. Mientras `b` no sea 0: cambia `(a, b)` por `(b, a mod b)` y cuenta una división.
2. Devuelve `a` y la cuenta.

## Coste

| Caso | Coste |
|---|---|
| Mejor | `O(1)`: una división cuando `b` divide a `a` |
| Medio y peor | `O(log min(a, b))` divisiones |
| Espacio extra | `O(1)`: dos números y un contador |

¿Por qué es tan rápido? Tras dos divisiones, el primer número del par es `a mod b`, que es menor que la mitad de `a` (con `a ≥ b`): si `b ≤ a / 2`, entonces `a mod b < b ≤ a / 2`; si no, `a mod b = a - b < a / 2`. Reducirse a la mitad cada dos pasos da una cuenta logarítmica. Los pares más lentos son los **números de Fibonacci vecinos**, donde casi todos los cocientes valen 1 y los números encogen lo menos posible: el ajuste "Fibonacci vecinos", 610 y 377, necesita 13 divisiones para llegar a `[1, 13]`.

## Errores comunes

- **Restar en lugar de dividir.** La forma más antigua resta `b` de `a` una y otra vez; con `a = 999` y `b = 1` son 999 pasos. El resto los hace todos de golpe.
- **Desbordamiento en el mcm.** `a · b` puede salirse del rango de los enteros aunque el mcm quepa; divide antes por el mcd.
- **Entradas con cero.** `gcd(a, 0) = a`, y el bucle ya lo resuelve. `gcd(0, 0)` sale 0, la convención habitual, aunque todo número divide a 0.

## Cuándo usarlo

Úsalo para simplificar fracciones, para encontrar la mayor baldosa o el mayor paso que encaja exactamente en dos longitudes, o como pieza para el mcm y los inversos modulares. El problema enlazado, Baldosa común, pide la mayor baldosa cuadrada que cubre un suelo exactamente: es el mcd de los dos lados.

## Temas relacionados

- [Inverso modular](learn:math-geometry/modular-inverse): las mismas divisiones, siguiendo además un coeficiente.
- [Criba de Eratóstenes](learn:math-geometry/sieve-eratosthenes): la divisibilidad vista desde los primos.
- [Curvas de crecimiento Big-O](learn:foundations/big-o-growth): qué significa en la práctica un número logarítmico de pasos.
