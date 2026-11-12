Una joyera graba pulseras con letras minúsculas alrededor de toda la pieza. Una pulsera no tiene principio, así que `"lamp"`, `"ampl"`, `"mpla"` y `"plam"` son la **misma** pulsera leída desde puntos distintos. Leerla al revés no vale: `"pmal"` es otra pulsera.

`codes` recoge los grabados tal como se apuntaron. Devuelve cuántas pulseras **distintas** hay.

### Ejemplo 1

```
codes = ["lamp", "mpla", "palm", "ampl", "lmpa"]
Salida: 2
```

`"lamp"`, `"mpla"` y `"ampl"` son una pulsera. `"palm"` y `"lmpa"` son otra: empieza a leer `"palm"` por su tercera letra.

### Ejemplo 2

```
codes = ["aa", "aa", "a"]
Salida: 2
```

Longitudes distintas son siempre pulseras distintas.

### Ejemplo 3

```
codes = ["abc", "bca", "cab", "cba"]
Salida: 2
```

### Restricciones

- 1 ≤ codes.length ≤ 2·10⁴
- 1 ≤ codes[i].length ≤ 12
- Los grabados solo contienen letras minúsculas de la `a` a la `z`
