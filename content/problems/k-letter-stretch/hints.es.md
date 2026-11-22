El ejemplo 1 pasa, pero el ejemplo 2 no. Prueba también un caso diminuto: `text = "aab", k = 1`. ¿Qué subcadena daría la longitud que devuelve el código y cuántas letras usa?
---
Añadir una letra, encoger la ventana y actualizar `best` están todos presentes. Fíjate en el orden en que ocurren dentro del bucle.
---
`best` se actualiza justo después de añadir una letra, antes de que el bucle `while` quite las letras sobrantes, así que se puede anotar una ventana con más de `k` letras distintas. Mueve la línea `best = max(...)` después del bucle que encoge la ventana.
