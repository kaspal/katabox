## Enfoque: contar mientras se recorre

Recorre el texto una vez. Pasa cada carácter a minúscula y comprueba si está en `"aeiou"`; si lo está, suma uno al contador. Los dígitos, espacios y signos nunca son vocales, y pasarlos a minúscula no los cambia.

**Complejidad:** O(n) en tiempo y O(1) de espacio extra.
