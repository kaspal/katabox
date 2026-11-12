Construye el cerebro de una calculadora de bolsillo. `expression` es una expresión aritmética formada por:

- números enteros no negativos (sin ceros a la izquierda, salvo el propio número `0`);
- los operadores `+`, `-` y `*`;
- paréntesis `(` y `)`;
- espacios, que no significan nada.

Se aplican las reglas de siempre: `*` liga más fuerte que `+` y `-`, los operadores del mismo tipo se aplican de izquierda a derecha y los paréntesis agrupan. Un `-` también puede ser un **menos unario** al principio de la expresión, después de `(` o justo después de otro operador (`2 * -3`, `-(1 + 2)`, `5 - -1`).

Devuelve el valor. Todos los resultados intermedios caben con holgura: su valor absoluto queda por debajo de 2⁵³.

### Ejemplo 1

```
expression = "2 + 3 * 4"
Salida: 14
```

### Ejemplo 2

```
expression = "(2 + 3) * 4"
Salida: 20
```

### Ejemplo 3

```
expression = "-(7 - 10) * -2"
Salida: -6
```

Los paréntesis dan −3, el menos unario lo convierte en 3, y 3 · −2 = −6.

### Ejemplo 4

```
expression = "10 - 2 - 3"
Salida: 5
```

De izquierda a derecha: (10 − 2) − 3.

### Restricciones

- 1 ≤ expression.length ≤ 10⁴
- La expresión es válida según lo descrito; los paréntesis y los signos menos unarios se anidan como mucho a 100 niveles
- Todo valor intermedio tiene valor absoluto < 2⁵³
