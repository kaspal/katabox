Esta vez la bolsa `b` va cambiando. Sus fichas están en fila, como la cadena `b`, y el jugador las cambia de una en una: el cambio `i` sustituye la ficha de la posición `positions[i]` por la letra `letters[i]`. Los cambios son permanentes y se acumulan.

Después de cada cambio, comprueba si las bolsas `a` y `b` son gemelas: cada letra aparece el mismo número de veces en las dos. Devuelve una respuesta por cambio, en orden. Dos bolsas con distinto número de fichas nunca son gemelas.

### Ejemplo 1

```
a = "tale", b = "late", positions = [0, 0, 3], letters = "plp"
Salida: [false, true, false]
```

`b` pasa a ser "pate", luego otra vez "late" y después "latp".

### Ejemplo 2

```
a = "acab", b = "bcaa", positions = [0, 3, 0], letters = "bbc"
Salida: [true, false, false]
```

El primer cambio pone una `b` donde ya había una.

### Ejemplo 3

```
a = "aaab", b = "cbbca", positions = [4, 0, 4], letters = "cab"
Salida: [false, false, false]
```

`b` tiene cinco fichas y `a` tiene cuatro, así que nunca son gemelas.

### Restricciones

- 1 ≤ a.length, b.length ≤ 10⁵
- 1 ≤ positions.length = letters.length ≤ 10⁵
- 0 ≤ positions[i] < b.length
- a, b y letters solo contienen letras minúsculas del alfabeto inglés.
