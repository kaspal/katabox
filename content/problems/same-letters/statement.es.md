En un juego de palabras cada jugador recibe una bolsa de fichas con letras. Dos bolsas son **gemelas** cuando contienen exactamente las mismas fichas: cada letra aparece el mismo número de veces en las dos, sin importar el orden.

Dadas las fichas de dos bolsas como cadenas `a` y `b`, devuelve `true` si las bolsas son gemelas y `false` si no.

### Ejemplo 1

```
a = "pastel", b = "plates"
Salida: true
```

### Ejemplo 2

```
a = "mood", b = "doom"
Salida: true
```

### Ejemplo 3

```
a = "tool", b = "lots"
Salida: false
```

`tool` tiene dos fichas `o` y ninguna `s`; `lots` tiene una `o` y una `s`.

### Restricciones

- 1 ≤ a.length, b.length ≤ 10⁵
- a y b solo contienen letras minúsculas del alfabeto inglés.
