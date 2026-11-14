No dividas el texto en palabras: los espacios, los signos y las líneas vacías tienen que salir justo donde estaban. Lee la primera línea como `s` y guarda todo lo que sigue como un único bloque de texto en el que solo cambias las letras.
---
Una letra minúscula `c` tiene la posición `p = c - 'a'` (de 0 a 25). Retrocederla `s` posiciones da la posición `(p - s + 26) mod 26`; suma de nuevo `'a'` para obtener la letra. Las mayúsculas funcionan igual con `'A'`.
---
Recorre el bloque carácter a carácter, descifra las letras con la fórmula anterior y copia todo lo demás (también los saltos de línea) sin cambios. Después escribe el bloque entero de una vez. Una tabla de traducción construida una sola vez (`bytes.maketrans`, `strtr`) hace ese mismo recorrido en una sola llamada.
