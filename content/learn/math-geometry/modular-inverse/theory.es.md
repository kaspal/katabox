## La idea

En la aritmética de siempre, dividir entre `a` es multiplicar por `1 / a`. Módulo `m` no hay fracciones, pero puede haber un número entero que haga el mismo papel: el **inverso modular** de `a` es el `x` de `[0, m)` con `a · x mod m = 1`. Una vez que lo tienes, "dividir entre `a`" pasa a ser "multiplicar por `x`".

Existe exactamente cuando `gcd(a, m) = 1`. La razón es la **identidad de Bézout**: las divisiones de Euclides se pueden recorrer hacia atrás para escribir el mcd como una combinación `a · x + m · y = gcd(a, m)` con números enteros `x` e `y`. Si el mcd es 1, leer esa ecuación módulo `m` elimina el término `m · y` y deja `a · x ≡ 1`. Si el mcd es algún `g > 1`, todo `a · x - m · y` es múltiplo de `g`, así que nunca puede valer 1, y no hay inverso.

El **algoritmo de Euclides extendido** encuentra `x` mientras hace las divisiones de siempre. Cada resto `r` lleva un coeficiente `x` con `r ≡ a · x (mod m)`. El par empieza como `a mod m` (coeficiente 1) y `m` (coeficiente 0, porque `m ≡ 0`). Cada división con cociente `q` forma un resto nuevo `old_r - q · r`, y la misma combinación de coeficientes, `old_x - q · x`, mantiene la regla. El lado de `y` nunca hace falta, así que el ejemplo no lo sigue. Cuando aparece el resto 0, `old_r` es el mcd: si vale 1, `old_x` es el inverso, llevado a `[0, m)`; si no, la salida es `[-1]`. El número y el módulo se escriben en dos campos.

### La entrada por defecto

`a = 17`, `m = 101` (el ajuste "un módulo primo"). Como pares `(resto, coeficiente)`: `(17, 1)` y `(101, 0)`. El primer cociente es 0, que solo los intercambia. Después, `101 = 5 · 17 + 16` da `(16, 0 - 5 · 1) = (16, -5)`; `17 = 1 · 16 + 1` da `(1, 1 - 1 · (-5)) = (1, 6)`; y `16 = 16 · 1 + 0` termina el bucle. Cuatro divisiones, mcd 1, coeficiente 6. La salida es `[6]`: `17 · 6 = 102 = 101 + 1`.

## Pasos

1. Empieza con `(old_r, r) = (a mod m, m)` y `(old_x, x) = (1, 0)`.
2. Mientras `r ≠ 0`: `q = old_r div r`; cambia `(old_r, r)` por `(r, old_r - q · r)` y `(old_x, x)` por `(x, old_x - q · x)`.
3. Si `old_r ≠ 1`, devuelve `-1`; si no, devuelve `old_x mod m`, como un valor de `[0, m)`.

## Coste

| Caso | Coste |
|---|---|
| Mejor | `O(1)`: cuando `a mod m` es 0 o 1, un par de divisiones lo resuelven |
| Medio y peor | `O(log m)`: las mismas divisiones que el algoritmo de Euclides |
| Espacio extra | `O(1)` |

## Errores comunes

- **Un resultado negativo.** El coeficiente suele ser negativo; en los lenguajes donde `%` conserva el signo, toma el resto, suma `m` y vuelve a tomarlo.
- **Dar por hecho que el inverso existe.** El ajuste "comparten un factor", 12 y 30, termina con mcd 6 y devuelve `[-1]`.
- **Fermat solo para primos.** Cuando `m` es primo, `a^(m - 2) mod m` también es el inverso (pequeño teorema de Fermat), calculado con [Potencia modular rápida](learn:math-geometry/fast-power). Con un módulo que no es primo, en general da una respuesta errónea; el algoritmo extendido funciona con cualquier `m`.

## Cuándo usarlo

Úsalo para dividir bajo un módulo: medias y probabilidades "módulo un primo", deshacer una multiplicación en un hash o calcular `C(n, k) = n! / (k! · (n - k)!)` módulo un primo a partir de tablas de factoriales. También resuelve `a · x ≡ b (mod m)`: multiplica `b` por el inverso.

## Temas relacionados

- [MCD de Euclides](learn:math-geometry/euclid-gcd): las mismas divisiones, sin el coeficiente.
- [Potencia modular rápida](learn:math-geometry/fast-power): el camino de Fermat para un módulo primo.
- [Triángulo de Pascal y C(n, k)](learn:math-geometry/pascal-ncr): coeficientes binomiales sin ninguna división.
