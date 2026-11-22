Imagina la temporada escrita dos veces seguidas. La respuesta de un día en el círculo es su respuesta en la primera copia de esa lista doble.
---
La pila de días que esperan del problema base también funciona sobre la lista doble, y no hace falta construirla: recorre `k` de 0 a `2n − 1` y mira el día `k mod n`.
---
Apila días solo en la primera vuelta (`k < n`); en la segunda, solo desapila. Cuando un día más cálido `i` desapila el día `d`, la espera es `(i − d) mod n`. Los días que sigan en la pila al final son los más cálidos de la temporada y se quedan con 0.
