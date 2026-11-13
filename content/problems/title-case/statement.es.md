El catálogo de una biblioteca tiene títulos escritos con todas las mezclas posibles de mayúsculas. Arregla un título para que cada **palabra** empiece con mayúscula y el resto de la palabra esté en minúscula.

Una palabra es un tramo máximo de caracteres que no son espacios. Los espacios se quedan exactamente donde están, también los repetidos. Una palabra que empieza con un dígito lo conserva, y sus letras pasan a minúscula.

Devuelve el título arreglado.

### Ejemplo 1

```
text = "the old MAN and the sea"
Salida: "The Old Man And The Sea"
```

### Ejemplo 2

```
text = "  two  spaces "
Salida: "  Two  Spaces "
```

### Ejemplo 3

```
text = "route 66 rules"
Salida: "Route 66 Rules"
```

### Restricciones

- 0 ≤ text.length ≤ 10⁴
- text solo tiene letras inglesas, dígitos y espacios.
