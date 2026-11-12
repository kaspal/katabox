Un reproductor de música guarda su lista de reproducción como una lista enlazada de identificadores de canciones, `playlist`, de la primera canción a la última. Su botón **Repetir la última** toma la última canción de la lista y la pasa al principio.

Pulsas el botón `k` veces. Devuelve la lista después de las pulsaciones. Una lista vacía sigue vacía.

Las listas se escriben como sus valores en orden: `[4, 8, 15]` es la lista 4 → 8 → 15, y `[]` es la lista vacía.

### Ejemplo 1

```
playlist = [4, 8, 15, 16, 23], k = 2
Salida: [16, 23, 4, 8, 15]
```

La primera pulsación pasa el 23 al principio; la segunda, el 16.

### Ejemplo 2

```
playlist = [7, 9], k = 5
Salida: [9, 7]
```

Cada dos pulsaciones esta lista vuelve a quedar como al principio.

### Ejemplo 3

```
playlist = [], k = 3
Salida: []
```

### Restricciones

- 0 ≤ número de canciones ≤ 10⁵
- 1 ≤ identificador de canción ≤ 10⁶
- 0 ≤ k ≤ 10⁹
