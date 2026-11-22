La tabla por cantidades del problema base sigue sirviendo, con "contar" en lugar de "el
menor". Pero si cada cantidad prueba cada ficha como la última, `1 + 2` y `2 + 1` se
cuentan dos veces. ¿Cómo consigues que cada combinación aparezca en un solo orden?
---
Decide las fichas valor a valor: primero cuántas del primer valor, luego cuántas del
segundo, y así sucesivamente. Así cada combinación se construye de una única forma.
---
Guarda `ways[a]`, empezando con `ways[0] = 1`. Recorre los valores de las fichas en el
bucle exterior; para cada valor `c`, recorre las cantidades desde `c` hasta `amount` y
suma `ways[a − c]` a `ways[a]`, módulo 1 000 000 007.
