Marco modera un foro de intercambio de cromos y filtra los identificadores de cromos vetados. Su filtro quita algunos, pero de vez en cuando alguno vetado se cuela en la lista final.

Dados un arreglo `items` y un arreglo `banned`, devuelve los elementos de `items` que no aparecen en `banned`, en su orden original. Si un valor permitido se repite, se conservan todas sus copias.

El editor empieza con ese código. Encuentra el error y corrígelo.

### Ejemplo 1

```
items = [4, 7, 7, 2, 9, 7], banned = [7]
Salida: [4, 2, 9]
```

Se eliminan todos los 7.

### Ejemplo 2

```
items = [49, 5, 17, 8, 32, 49], banned = [31, 42, 25]
Salida: [49, 5, 17, 8, 32, 49]
```

Ningún elemento de `items` está vetado.

### Ejemplo 3

```
items = [1, 2, 1, 3, 3, 2], banned = [3, 1]
Salida: [2, 2]
```

### Restricciones

- 1 ≤ items.length ≤ 10⁵
- 0 ≤ banned.length ≤ 10⁵
- 1 ≤ items[i], banned[j] ≤ 10⁹
