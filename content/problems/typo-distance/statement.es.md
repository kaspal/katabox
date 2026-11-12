Un corrector ortográfico ordena sus sugerencias según cuántos arreglos de teclado separan lo que escribiste, `typed`, de la palabra que querías, `meant`. Un arreglo es uno de estos:

- **insertar** una letra en cualquier sitio,
- **borrar** una letra,
- **sustituir** una letra por otra.

Devuelve el menor número de arreglos que convierten `typed` en `meant`.

### Ejemplo 1

```
typed = "cofee", meant = "coffee"
Salida: 1
```

Insertar una `f`.

### Ejemplo 2

```
typed = "stake", meant = "skate"
Salida: 2
```

Sustituir la `t` por `k` y la `k` por `t`.

### Ejemplo 3

```
typed = "lamp", meant = "pal"
Salida: 3
```

Una forma: sustituir la `l` por `p` (`pamp`), borrar la `m` (`pap`) y sustituir la última `p` por `l` (`pal`).

### Restricciones

- 1 ≤ typed.length, meant.length ≤ 1000
- typed y meant solo contienen letras minúsculas del alfabeto inglés.
