## Idea

Reduce el problema al base. Para un objeto con `c` copias, córtalas en paquetes de 1, 2,
4, ..., 2ᵏ⁻¹ copias, mientras quepan en `c`, y un último paquete con el resto. Cualquier
número de copias entre 0 y `c` es la suma de algunos de estos paquetes, y ninguna elección
de paquetes pasa de `c`. Así, llevar cada paquete como mucho una vez (un objeto 0/1 con el
peso y el valor totales del paquete) permite exactamente las elecciones que permite el
problema.

Después usa la tabla del problema base: `best[cap]` es el mejor valor con peso como mucho
`cap`, y cada paquete de peso `W` y valor `V` actualiza
`best[cap] = max(best[cap], best[cap − W] + V)` para `cap` desde `capacity` hasta `W`.

## Complejidad

O(capacity · Σ log counts[i]) en tiempo: con como mucho 7 paquetes por objeto, unos
5000 · 50 · 7 ≈ 1,75·10⁶ pasos. O(capacity) de memoria extra.
