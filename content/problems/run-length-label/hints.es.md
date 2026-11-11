Recorre `text` una sola vez. ¿Cómo sabes que una racha acaba de terminar?
---
Una racha termina cuando la siguiente letra es distinta o cuando llegas al final del texto. Guarda la letra de la racha actual y su longitud hasta el momento.
---
Cuando termine una racha, añade su letra y luego su longitud si es al menos 2. Construye el resultado en una lista (o un constructor de cadenas) y únelo una sola vez: concatenar cadenas repetidamente puede ser lento.
