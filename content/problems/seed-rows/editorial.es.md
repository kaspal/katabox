## Enfoque: programación dinámica sobre (longitud llena, última clase)

Sea `formas[x][k]` el número de plantaciones válidas de exactamente `x` centímetros cuya **última** planta es de la clase `k`, y `total[x] = Σ formas[x][k]`, con `total[0] = 1` para el bancal vacío (y `formas[0][k] = 0`).

Toma una plantación contada en `formas[x][k]` y quítale la última planta. Lo que queda llena `x − w` centímetros (`w = widths[k]`), es válido por sí mismo y no termina en la clase `k`, porque eso juntaría dos `k`. Al revés, añadir una planta de clase `k` a cualquier plantación así da una plantación contada en `formas[x][k]`. Por tanto

```
formas[x][k] = total[x − w] − formas[x − w][k]      (0 si x < w)
total[x]     = Σ sobre k de formas[x][k]
```

El bancal vacío no tiene última planta, así que `total[0] − formas[0][k] = 1` permite correctamente una plantación de una sola planta. Calcula `x` de 1 a `length`; la respuesta es `total[length]`. Mantén cada número módulo 1 000 000 007 (suma el módulo antes de restar para que no salga negativo).

### Paso a paso

`widths = [1, 2]` (A, B). `total[0] = 1`. x = 1: A = 1, B = 0, total 1. x = 2: A = total[1] − A[1] = 0, B = total[0] − 0 = 1, total 1. x = 3: A = 1 − 0 = 1 (BA), B = 1 − 0 = 1 (AB), total 2. x = 4: A = 2 − 1 = 1 (ABA), B = 1 − 1 = 0, total 1.

**Complejidad:** O(length · k) de tiempo y memoria (u O(anchuraMáxima · k) de memoria con una ventana deslizante). Probar cada plantación recursivamente cuesta un tiempo exponencial.
