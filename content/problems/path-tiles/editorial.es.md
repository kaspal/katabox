## Enfoque: contar según la última pieza

Todo pavimento de un sendero de longitud `i` termina en exactamente una de las tres piezas, y quitarla deja un pavimento de un sendero más corto:

```
formas[0] = 1                       # el sendero vacío: una forma (no poner nada)
formas[i] = formas[i − 1]           # termina en una cuadrada
          + formas[i − 2]           # termina en una losa corta  (si i ≥ 2)
          + formas[i − 3]           # termina en una losa larga  (si i ≥ 3)
```

Últimas piezas distintas dan secuencias distintas, y cada secuencia se cuenta una vez, así que la suma es exacta. Calcula `i` de 1 a `n`, guardando cada valor módulo 1 000 000 007; en cada momento solo hacen falta los tres últimos valores.

### Paso a paso

`formas = 1, 1, 2, 4, 7, 13` para longitudes de 0 a 5: por ejemplo `formas[4] = 4 + 2 + 1 = 7` y `formas[5] = 7 + 4 + 2 = 13`.

**Complejidad:** O(n) de tiempo y O(1) de memoria con tres valores. La recursión simple recalcula las mismas longitudes y crece como 1,84ⁿ.
