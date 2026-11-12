Un huerto comunitario planta un bancal recto de `length` centímetros, una planta detrás de otra. Hay varias **clases** de planta: una planta de la clase `i` ocupa exactamente `widths[i]` centímetros. Dos clases pueden necesitar la misma anchura y siguen siendo clases distintas.

El bancal tiene que quedar **exactamente** lleno, sin hueco al final, y dos plantas de la **misma clase** no pueden quedar juntas (las plagas se propagan con demasiada facilidad). Por lo demás, las clases se pueden usar las veces que se quiera.

¿Cuántas plantaciones distintas hay? Dos plantaciones son distintas si las secuencias de clases son distintas. El número puede ser enorme: devuélvelo módulo 1 000 000 007.

### Ejemplo 1

```
length = 4, widths = [1, 2]
Salida: 1
```

Con las clases A (1 cm) y B (2 cm), solo A B A llena 4 cm sin dos vecinas iguales: B B y A A A A repiten clase, y A A B o B A A juntan dos A.

### Ejemplo 2

```
length = 3, widths = [1, 1, 1]
Salida: 12
```

Tres clases de anchura 1: 3 opciones para la primera planta y 2 para cada una de las dos siguientes.

### Ejemplo 3

```
length = 1, widths = [2]
Salida: 0
```

### Restricciones

- 1 ≤ length ≤ 10⁴
- 1 ≤ widths.length ≤ 50
- 1 ≤ widths[i] ≤ 100
