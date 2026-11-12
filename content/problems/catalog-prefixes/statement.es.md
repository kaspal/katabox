Una pequeña biblioteca prueba un buscador nuevo. Mientras un lector escribe, el buscador muestra cuántos títulos del catálogo **empiezan por** las letras escritas hasta ese momento.

`titles` es el catálogo (un título puede aparecer más de una vez: cada copia cuenta) y `typed` recoge lo que escribieron los lectores. Para cada entrada de `typed`, devuelve cuántos títulos empiezan por ella. Un título empieza por sí mismo.

### Ejemplo 1

```
titles = ["moon", "moonlight", "mars", "moon", "map"], typed = ["mo", "m", "moonl", "x"]
Salida: [3, 5, 1, 0]
```

"mo" coincide con las dos copias de "moon" y con "moonlight"; todos los títulos empiezan por "m"; solo "moonlight" empieza por "moonl".

### Ejemplo 2

```
titles = ["river"], typed = ["river", "rivers", "r"]
Salida: [1, 0, 1]
```

"rivers" es más largo que el único título, así que ninguno empieza por él.

### Ejemplo 3

```
titles = ["ab", "abc", "b"], typed = ["abc", "ab", "a"]
Salida: [1, 2, 2]
```

### Restricciones

- 1 ≤ titles.length ≤ 10⁴
- 1 ≤ typed.length ≤ 2·10⁴
- 1 ≤ titles[i].length, typed[j].length ≤ 20
- Los títulos y el texto escrito solo usan letras minúsculas del alfabeto inglés
