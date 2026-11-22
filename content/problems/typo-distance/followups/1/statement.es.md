Ahora el corrector solo necesita un sí o un no. Un arreglo sigue siendo uno de estos:
**insertar** una letra en cualquier sitio, **borrar** una letra o **sustituir** una letra
por otra. Devuelve `true` si `typed` se puede convertir en `meant` con **como mucho `k`
arreglos**, y `false` si no.

Las palabras son mucho más largas que antes (hasta 2·10⁴ letras), pero `k` es pequeño.

### Ejemplo 1

```
typed = "lantern", meant = "planter", k = 2
Salida: true
```

Insertar una `p` al principio y borrar la `n` final: exactamente 2 arreglos.

### Ejemplo 2

```
typed = "eacadd", meant = "edcadd", k = 2
Salida: true
```

### Ejemplo 3

```
typed = "aacbc", meant = "aacbbb", k = 2
Salida: true
```

Sustituir la última `c` por `b` e insertar otra `b`.

### Restricciones

- 1 ≤ typed.length, meant.length ≤ 2·10⁴
- typed y meant solo contienen letras minúsculas del alfabeto inglés.
- 0 ≤ k ≤ 20
