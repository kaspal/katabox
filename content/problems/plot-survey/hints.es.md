Probar cada punto de coordenadas enteras de la parcela es inútil: una parcela puede contener unos 10¹² de ellos. Busca fórmulas que usen solo las `n` esquinas: una para el área y otra que relacione el área con los puntos interiores.
---
La **fórmula del área de Gauss** (shoelace) da el doble del área como número entero: `|Σ (xᵢ·yᵢ₊₁ − xᵢ₊₁·yᵢ)|`, recorriendo las esquinas. El **teorema de Pick** relaciona el área `A` de un polígono con esquinas enteras con sus puntos interiores `I` y los puntos enteros sobre su cerca `B`: `A = I + B/2 − 1`.
---
Para cada lado de `(x₁, y₁)` a `(x₂, y₂)`, suma `x₁·y₂ − x₂·y₁` a `twice` y `mcd(|x₂ − x₁|, |y₂ − y₁|)` a `B` (los puntos de cerca de ese lado, contando uno de sus extremos). Después `twice = |twice|`, el área es `twice / 2` (`.5` cuando `twice` es impar) e `I = (twice − B + 2) / 2`.
