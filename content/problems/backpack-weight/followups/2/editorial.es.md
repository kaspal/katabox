## Idea

Intercambia los papeles del peso y el valor. Sea `S` la suma de todos los valores (como
mucho 10⁴) y `lightest[v]` el menor peso de un grupo de objetos que vale exactamente `v`,
empezando con `lightest[0] = 0` y el resto inalcanzable. Cada objeto de peso `w` y valor
`x` actualiza `lightest[v] = min(lightest[v], lightest[v − x] + w)` para `v` desde `S`
hasta `x`; ir hacia abajo usa cada objeto como mucho una vez, igual que en el problema
base. Al final, busca el mayor `v` cuyo `lightest[v]` cabe en `capacity`; `v = 0` siempre
cabe.

El peso total de todos los objetos es como mucho 100 · 10⁷ = 10⁹, así que cualquier suma
cabe en un entero de 32 bits con signo.

## Complejidad

O(n · S) en tiempo, como mucho 100 · 10⁴ = 10⁶ pasos, y O(S) de memoria extra, sea cual
sea la capacidad.
