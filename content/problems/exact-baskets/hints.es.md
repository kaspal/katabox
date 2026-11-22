Prueba el caso más pequeño cuya respuesta no use el tarro más ligero: `weights = [1, 2]`, `k = 1`, `target = 2`. Luego imprime `pick` al principio de cada llamada a `go`.
---
La condición de parada y el total acumulado están bien. Mira qué le pasa a `pick` dentro del bucle `for` cuando vuelve la llamada recursiva: ¿es la misma lista que antes de hacer la elección?
---
Cada elección se añade a `pick` pero nunca se deshace, así que la segunda opción del bucle se prueba encima de la primera, y `pick` pronto supera los `k` elementos y ya no vuelve a coincidir. Deshaz la elección justo después de la llamada recursiva: `pick.pop()`.
