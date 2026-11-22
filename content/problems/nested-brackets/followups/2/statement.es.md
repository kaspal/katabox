Esta vez `text` solo contiene corchetes, `()[]{}`, sin letras, y en conjunto no tiene por qué estar bien anidado. Encuentra la **parte** más larga, una serie de caracteres consecutivos, que esté bien anidada por sí sola, con la misma regla de antes: cada corchete que abre se cierra más adelante con uno del mismo tipo, y un par que se abre dentro de otro también se cierra dentro de él.

Devuelve la longitud de esa parte, o `0` si ninguna parte está equilibrada (una parte vacía no cuenta).

### Ejemplo 1

```
text = "](()[]){[}"
Salida: 6
```

La parte `(()[])`, de la posición 1 a la 6.

### Ejemplo 2

```
text = "(){}[]["
Salida: 6
```

Los seis primeros caracteres; el último `[` nunca se cierra.

### Ejemplo 3

```
text = "(()))("
Salida: 4
```

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- `text` solo contiene `()[]{}`.
