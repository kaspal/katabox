El mejor cartel está limitado por su panel más bajo. Si supieras qué panel es el más bajo bajo el cartel, ¿cuánto de ancho podría ser el cartel?
---
Con el panel `k` como el más bajo, el cartel se extiende a izquierda y derecha hasta el primer panel **más bajo** que `panels[k]` en cada lado. Necesitas, para cada panel, el panel más bajo más cercano a cada lado.
---
Una pila de posiciones de paneles con alturas crecientes encuentra los dos a la vez: cuando el panel `i` es más bajo que la cima de la pila, sácala; el cartel del panel sacado termina justo antes de `i` y empieza justo después del panel que queda en la cima.
