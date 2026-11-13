Una tienda de pancartas vende tiras de letras que se leen igual de izquierda a derecha que al revés (**palíndromos**). Un cliente quiere la pancarta `text`, así que hay que cortarla en piezas que sean palíndromos; una sola letra siempre lo es.

Devuelve el menor número de cortes necesario.

### Ejemplo 1

```
text = "noonabc"
Salida: 3
```

"noon" | "a" | "b" | "c".

### Ejemplo 2

```
text = "racecar"
Salida: 0
```

### Ejemplo 3

```
text = "abab"
Salida: 1
```

"aba" | "b".

### Restricciones

- 1 ≤ text.length ≤ 1500
- text solo tiene letras minúsculas inglesas.
