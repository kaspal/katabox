Los costes nunca son negativos. Si un tramo se pasa del presupuesto, ¿qué ocurre si lo alargas?
---
Sigue pasándose. Así que, para cada etapa final, el mejor inicio solo se mueve hacia la derecha a medida que avanza el final.
---
Mantén una ventana `[left, right]` y su total. Suma `costs[right]`; mientras el total supere `budget`, resta `costs[left]` y mueve `left` a la derecha. Entonces la ventana es válida: compara su longitud con la mejor hasta ahora.
