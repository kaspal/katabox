Un calendario compartido guarda las horas ocupadas de una persona en `slots`, cada una `[start, end]` con los dos momentos incluidos. Las franjas están ordenadas por inicio y no hay dos que compartan un momento. Llega una nueva franja ocupada, `fresh`.

Insértala y devuelve la lista actualizada con la misma forma: ordenada, y con cada par de franjas que comparten al menos un momento (aunque solo sea un extremo) combinadas en una.

### Ejemplo 1

```
slots = [[1, 3], [6, 8], [10, 12]], fresh = [7, 10]
Salida: [[1, 3], [6, 12]]
```

[7, 10] se solapa con [6, 8] y toca [10, 12] en el momento 10.

### Ejemplo 2

```
slots = [], fresh = [4, 5]
Salida: [[4, 5]]
```

### Ejemplo 3

```
slots = [[2, 4]], fresh = [5, 6]
Salida: [[2, 4], [5, 6]]
```

### Ejemplo 4

```
slots = [[1, 2], [5, 9]], fresh = [2, 5]
Salida: [[1, 9]]
```

### Restricciones

- 0 ≤ slots.length ≤ 10⁴
- 0 ≤ start ≤ end ≤ 10⁶ para cada franja y para `fresh`
- `slots` está ordenada por inicio, y cada franja empieza después de que termine la anterior.
