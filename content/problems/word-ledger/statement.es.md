Los socios de un club de observación de aves escriben sus notas de campo en un cuaderno compartido, que se pasa a limpio al final de la temporada. Para la portada del boletín anual, la editora quiere las palabras que más se repiten en esas notas, junto con cuántas veces aparece cada una.

Este es un **problema de programa**: lee las notas de la entrada estándar y escribe las palabras más frecuentes en la salida estándar.

## Formato de entrada

- La primera línea contiene `K`, cuántas palabras quiere la editora.
- Todas las líneas siguientes, hasta el final de la entrada, son texto del cuaderno. Una **palabra** es una secuencia máxima de letras minúsculas `a`-`z`. Las palabras se separan con espacios y con los signos `.` `,` `;` `:` `!` `?`, en cualquier cantidad, y una línea puede estar vacía. No se indica cuántas líneas de texto hay: lee hasta que se acabe la entrada.

## Formato de salida

Escribe las `K` palabras más frecuentes, una por línea como `palabra cantidad`: primero la más frecuente, y las que tengan la misma cantidad en orden alfabético. Si las notas tienen menos de `K` palabras distintas, escríbelas todas.

### Ejemplo 1

Entrada:

```
2
the heron saw the duck.
the duck flew!
```

Salida:

```
the 3
duck 2
```

### Ejemplo 2

Entrada:

```
3
wren, owl; jay: wren?
owl
```

Salida:

```
owl 2
wren 2
jay 1
```

`owl` y `wren` empatan, así que van en orden alfabético.

### Ejemplo 3

Entrada:

```
5
coot coot coot
```

Salida:

```
coot 3
```

### Ejemplo 4

Entrada:

```
2
rain,rain!sun

sun...rain
```

Salida:

```
rain 3
sun 2
```

Los signos de puntuación separan palabras aunque no haya espacio, y la línea vacía no tiene palabras.

### Restricciones

- 1 ≤ K ≤ 100
- El texto que sigue a la primera línea tiene como mucho 200 000 caracteres, todos letras minúsculas, espacios, saltos de línea o `.,;:!?`.
- El texto contiene al menos una palabra.
