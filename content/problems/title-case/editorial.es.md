## Enfoque: recordar el carácter anterior

Recorre el texto una vez con una marca `alInicio` que es verdadera al principio y después de cada espacio:

- un espacio se copia y pone `alInicio = true`;
- cualquier otro carácter se pasa a mayúscula si `alInicio` y a minúscula en caso contrario, y pone `alInicio = false`.

Pasar un dígito a mayúscula o minúscula no lo cambia, así que una palabra como `66` se queda igual y `3RD` pasa a `3rd`.

Separar por espacios y volver a unir perdería los espacios repetidos, que el enunciado pide conservar.

**Complejidad:** O(n) en tiempo, O(n) para el resultado.
