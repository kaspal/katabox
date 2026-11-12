Recorrer hacia delante desde cada día funciona, pero una racha fría larga hace que cada día recorra casi hasta el final: O(n²).
---
Recorre los días en orden y guarda los que siguen esperando uno más cálido. Cuando llega un día cálido, ¿a qué días en espera responde?
---
Mantén una pila de índices cuyas temperaturas nunca suben de abajo arriba. Para cada día `i`, saca todo índice `j` con `temps[j] < temps[i]` y pon `wait[j] = i - j`; después apila `i`. Lo que quede en la pila espera para siempre (0).
