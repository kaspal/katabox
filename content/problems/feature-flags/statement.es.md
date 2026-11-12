Una app tiene 16 funciones opcionales, numeradas del 0 al 15. La configuración de cada usuario se guarda como un número cuyo bit `f` vale 1 cuando la función `f` está activada: `users[i]` es el número del usuario `i`.

El equipo de producto hace preguntas como "¿cuántos usuarios tienen activadas **todas** estas funciones (y quizá otras)?". Cada pregunta también es un número, con el bit `f` activo para cada función por la que pregunta. Para cada pregunta de `queries`, devuelve cuántos usuarios tienen activadas todas las funciones preguntadas.

### Ejemplo 1

```
users = [5, 7, 2, 15], queries = [5, 2, 8, 0]
Salida: [3, 3, 1, 4]
```

La pregunta 5 (en binario 0101) pregunta por las funciones 0 y 2: los usuarios 5, 7 y 15 tienen las dos. La pregunta 0 no pregunta por nada, así que cuentan todos los usuarios.

### Ejemplo 2

```
users = [0, 0], queries = [1]
Salida: [0]
```

### Ejemplo 3

```
users = [65535], queries = [65535, 32768]
Salida: [1, 1]
```

### Restricciones

- 1 ≤ users.length, queries.length ≤ 10⁵
- 0 ≤ users[i], queries[j] < 2¹⁶
