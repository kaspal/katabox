Pasa por un entero normal: primero lee el número y después vuelve a escribirlo.
---
Leer, de izquierda a derecha: `valor = valor · source + dígito`. Escribir: el último dígito en base `target` es `valor mod target`; después divide entre `target` y repite.
---
Al escribir, los dígitos salen de derecha a izquierda, así que dales la vuelta al final, y trata el 0 aparte (el bucle no escribiría nada).
