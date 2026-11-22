Mateo analiza cantos de pájaros escritos como cadenas de letras, una letra por tipo de nota, y quiere el tramo más largo de un canto que use como mucho `k` notas distintas. Su función devuelve números creíbles, pero no está seguro de que acierte con todos los cantos.

Dados `text` y `k`, devuelve la longitud de la subcadena (una parte contigua) más larga de `text` que contenga como mucho `k` letras distintas.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
text = "abaccc", k = 2
Salida: 4
```

"accc" solo usa las letras a y c.

### Ejemplo 2

```
text = "bebhhhgdb", k = 2
Salida: 4
```

### Ejemplo 3

```
text = "abaccbbca", k = 3
Salida: 9
```

La cadena entera usa tres letras.

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- text está formado por letras minúsculas del alfabeto inglés.
- 1 ≤ k ≤ 26
