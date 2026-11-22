Volver a contar las dos bolsas después de cada cambio cuesta O(n) cada vez, hasta 10¹⁰ pasos en total. Un cambio solo toca dos letras: la que sale y la que entra.
---
Guarda, para cada una de las 26 letras, su cuenta en `a` menos su cuenta en `b`. Las bolsas son gemelas justo cuando las 26 diferencias valen 0.
---
Guarda también cuántas letras tienen una diferencia distinta de cero, y guarda `b` como un arreglo para saber qué letra quita cada cambio. En cada cambio, actualiza la diferencia de la ficha vieja y la de la nueva, y ajusta el contador cada vez que una diferencia pasa a ser cero o deja de serlo. Responde `true` cuando el contador valga 0.
