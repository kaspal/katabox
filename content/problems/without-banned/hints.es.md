Prueba dos valores vetados seguidos, como `items = [7, 7]` con `banned = [7]`. ¿Cuántos sobreviven?
---
La comprobación de pertenencia está bien. Piensa en qué le pasa al bucle cuando la lista que recorre cambia mientras la recorre.
---
Al eliminar un elemento, todos los que van detrás se desplazan una posición a la izquierda, así que el bucle se salta el que ocupa la posición actual. No modifiques `items` mientras lo recorres; construye una lista nueva con los valores permitidos (en TypeScript, usa `filter`).
