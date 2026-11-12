El orden del tablón compara a los corredores por dos cosas: primero el tiempo y, solo si los tiempos son iguales, el nombre.
---
Ordena los índices de los corredores (o parejas `(tiempo, nombre)`) en vez de ordenar las dos listas por separado, para que cada nombre siga con su tiempo.
---
La mayoría de los lenguajes ordenan tuplas o parejas lexicográficamente: ordenar por la clave `(tiempo, nombre)` hace justo lo que pide el tablón. Si no, escribe un comparador: compara los tiempos y, solo si empatan, los nombres.
