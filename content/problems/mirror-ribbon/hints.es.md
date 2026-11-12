Podrías construir una copia limpia del texto y compararla con su reverso. ¿Puedes evitar construir ninguna copia?
---
Lee desde los dos extremos a la vez. ¿Qué debe pasar cuando el carácter de un extremo es un espacio o un símbolo?
---
Mantén `i` al principio y `j` al final. Salta en cada lado los caracteres que no sean letras ni dígitos; luego compara los dos caracteres en minúscula. Si no coinciden, `false`; si coinciden, acerca ambos índices hasta que se crucen.
