Lee toda la entrada de una vez y divídela en tokens. Lee `T` y, para cada sección, lee `n` y toma los `n` tokens siguientes como sus alturas.
---
Con las alturas de una sección ordenadas, la mediana es simplemente la que está en una posición fija. ¿Qué posición es para `n = 5`? ¿Y para `n = 4`, donde quieres el más bajo de los dos del medio?
---
Ordena las alturas como números y escribe `heights[(n − 1) / 2]` (desde 0, división entera): la posición 2 para `n = 5`, la 1 para `n = 4`, la 0 para `n = 1`. Reúne las respuestas y escríbelas de una vez al final.
