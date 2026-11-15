Un pequeño editor de pixel art dibuja con caracteres: cada carácter es un píxel. Su herramienta de zoom amplía un dibujo por un factor entero `K`, convirtiendo cada píxel en un bloque de `K × K` copias del mismo carácter, de modo que el dibujo conserva su forma y queda `K` veces más ancho y `K` veces más alto.

Este es un **problema de programa**: lee el dibujo y el factor de zoom de la entrada estándar y escribe el dibujo ampliado en la salida estándar.

## Formato de entrada

- La primera línea contiene `R C K`: el número de filas, el número de columnas y el factor de zoom.
- Después, `R` líneas, las filas del dibujo de arriba abajo, cada una con exactamente `C` caracteres. Cada carácter es uno de `.`, `#`, `*`, `o` y `+`.

## Formato de salida

Escribe el dibujo ampliado: `R · K` líneas de `C · K` caracteres cada una.

### Ejemplo 1

Entrada:

```
2 3 2
#.#
.o.
```

Salida:

```
##..##
##..##
..oo..
..oo..
```

Cada píxel se convierte en un bloque de 2 × 2: la primera fila `#.#` pasa a ser dos copias de `##..##`.

### Ejemplo 2

Entrada:

```
1 1 3
*
```

Salida:

```
***
***
***
```

### Ejemplo 3

Entrada:

```
3 2 1
+.
.+
##
```

Salida:

```
+.
.+
##
```

Con `K = 1` el dibujo queda igual.

### Restricciones

- 1 ≤ R ≤ 40
- 1 ≤ C ≤ 40
- 1 ≤ K ≤ 8
- Cada fila tiene exactamente `C` caracteres de entre `.`, `#`, `*`, `o`, `+`.
