Un festival imprime mensajes en cintas largas que la gente lee desde cualquiera de los dos extremos. Una cinta es una **cinta espejo** cuando sus letras y dígitos se leen igual de izquierda a derecha que de derecha a izquierda. Se ignoran los espacios, la puntuación y cualquier otro símbolo, y también la diferencia entre mayúsculas y minúsculas.

Dado el texto de la cinta, `text`, devuelve `true` si es una cinta espejo y `false` si no. Una cinta sin ninguna letra ni dígito es una cinta espejo.

### Ejemplo 1

```
text = "Stack cats, 2 by 2!"
Salida: false
```

Si solo dejamos letras y dígitos en minúscula queda `stackcats2by2`, que al revés es `2yb2stackcats`.

### Ejemplo 2

```
text = "Top spot: 7 -- 7 tops pot"
Salida: true
```

Las letras y dígitos son `topspot77topspot`, igual en los dos sentidos.

### Ejemplo 3

```
text = "...!"
Salida: true
```

### Restricciones

- 1 ≤ text.length ≤ 10⁵
- text contiene caracteres ASCII imprimibles (códigos 32 a 126).
