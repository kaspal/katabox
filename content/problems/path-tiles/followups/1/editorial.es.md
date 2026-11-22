## Idea

La cuenta sigue cumpliendo `f(i) = f(i − 1) + f(i − 2) + f(i − 3)` con `f(0) = 1` (el
sendero vacío), pero `n` es demasiado grande para avanzar paso a paso. Escribe un paso
como una matriz:

```
[f(i+1)]   [1 1 1] [f(i)  ]
[f(i)  ] = [1 0 0] [f(i-1)]
[f(i-1)]   [0 1 0] [f(i-2)]
```

así que `n` pasos son esa matriz elevada a la `n`, aplicada a `(1, 0, 0)`. La
exponenciación rápida calcula la potencia con unos 2·log₂ n productos de matrices, cada
uno reducido módulo 1 000 003. El módulo pequeño mantiene cada entrada por debajo de unos
10⁶, así que el producto de dos entradas queda por debajo de 10¹², y la suma de tres es
exacta incluso en un número de JavaScript.

## Complejidad

O(log n) en tiempo: unas 50 elevaciones al cuadrado y hasta 50 productos más de matrices
de 3×3 para n = 10¹⁵. O(1) de memoria extra.
