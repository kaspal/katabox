El texto de un nodo contiene los textos de sus hijos: encaja una función recursiva.
---
Escribe el valor y decide la parte izquierda: hace falta cuando hay hijo izquierdo, **o** cuando hay hijo derecho (como `()`).
---
`texto(nodo) = str(val) + ("(" + texto(izq) + ")" si izq o der) + ("(" + texto(der) + ")" si der)`. En árboles largos, construye la cadena en una lista y únela al final.
