## Enfoque: fórmula del área de Gauss y teorema de Pick

El método obvio prueba cada punto entero del rectángulo que envuelve la parcela contra la cerca. Una parcela puede ocupar todo el campo, así que ese rectángulo contiene hasta 2·10⁶ × 2·10⁶ = 4·10¹² puntos, cada uno comparado con hasta 1000 lados: imposible. Su coste depende del área, no de `n`. Dos fórmulas clásicas solo necesitan las esquinas, así que el trabajo es O(n).

**El área: fórmula de Gauss (shoelace).** Recorriendo las esquinas, `x₁·y₂ − x₂·y₁` es el doble del área con signo del triángulo que forman el origen y el lado de `(x₁, y₁)` a `(x₂, y₂)`. Al sumar sobre todos los lados, las partes fuera de la parcela se cancelan y queda el doble del área de la parcela, positivo si las esquinas van en sentido antihorario y negativo si van en sentido horario:

```
twice = |Σ (xᵢ·yᵢ₊₁ − xᵢ₊₁·yᵢ)|      (la esquina siguiente a la última es la primera)
```

`twice` es un número entero, así que el área es `twice / 2` y termina en `.5` exactamente cuando `twice` es impar. Escríbela a partir del entero (`twice // 2`, luego `.0` o `.5`) y los decimales en coma flotante no intervienen nunca.

**Los puntos de la cerca.** En un lado con desplazamientos `dx = x₂ − x₁` y `dy = y₂ − y₁`, los puntos enteros están separados por `(dx/g, dy/g)`, donde `g = mcd(|dx|, |dy|)`: hay `g + 1` contando los dos extremos. Contando un extremo por lado (el otro es el inicio del lado siguiente) sale `B = Σ mcd(|dx|, |dy|)`. Una esquina en un tramo recto no da problemas: sus dos lados simplemente se reparten esos puntos.

**Los puntos interiores: teorema de Pick.** Para un polígono simple cuyas esquinas son puntos enteros, `A = I + B/2 − 1`. Despejando `I`:

```
I = (twice − B + 2) / 2
```

Ejemplo 2: `twice = |0 + 15 + 0| = 15`, así que el área es 7.5. Los lados dan `B = mcd(5, 0) + mcd(5, 3) + mcd(0, 3) = 5 + 1 + 3 = 9`, e `I = (15 − 9 + 2) / 2 = 4`.

**Por qué es correcto:** la suma de Gauss es exacta para cualquier polígono simple, en cualquier sentido (el valor absoluto quita el signo). El teorema de Pick vale para todo polígono simple con esquinas enteras, sea convexo o no; las restricciones garantizan justo eso, así que `twice − B + 2` siempre es par.

**Complejidad:** tiempo O(n log C) por los `n` mcd (con `C` = 2·10⁶), memoria O(n).

**Tamaño de los números:** cada término `x₁·y₂ − x₂·y₁` mide como mucho 2·10¹² en valor absoluto, y `twice` como mucho 8·10¹². Usa `i64` en Rust. En TypeScript todos los valores quedan muy por debajo de 2⁵³, así que `Number` es exacto; los enteros de PHP son de 64 bits. Rust, TypeScript y PHP no traen un mcd para enteros normales: escribe el bucle de Euclides (`mcd(a, 0) = a`, y ningún lado es `(0, 0)` porque las esquinas son distintas).

**Entrada:** lee todos los tokens de una vez (`sys.stdin.buffer.read().split()` y equivalentes).
