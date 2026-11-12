Un club de escalada imprime los nombres de sus socios en taquillas pequeñas, y las etiquetas son diminutas. Para cada nombre, el club quiere el **comienzo más corto** por el que no empiece el nombre de ningún otro socio, para que la etiqueta siga señalando a un único socio.

Si incluso el nombre entero es el comienzo del nombre de otro socio (como "cat" y "cattle"), la etiqueta es el nombre entero.

`names` recoge a los socios (todos los nombres son distintos). Devuelve las etiquetas en el mismo orden.

### Ejemplo 1

```
names = ["dog", "door", "cat", "cattle"]
Salida: ["dog", "doo", "cat", "catt"]
```

"do" también es el comienzo de "door", así que "dog" necesita sus tres letras. "cat" es el comienzo de "cattle", así que su etiqueta es el nombre entero; "cattle" se distingue con "catt".

### Ejemplo 2

```
names = ["zebra"]
Salida: ["z"]
```

### Ejemplo 3

```
names = ["ab", "b", "abc"]
Salida: ["ab", "b", "abc"]
```

### Restricciones

- 1 ≤ names.length ≤ 2·10⁴
- 1 ≤ names[i].length ≤ 20, letras minúsculas del alfabeto inglés
- Todos los nombres son distintos
