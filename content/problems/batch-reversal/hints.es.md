Antes de invertir un lote, asegúrate de que está completo: mira k nodos por delante.
---
Invertir un lote sin memoria extra necesita tres punteros. Después, el antiguo primer nodo del lote es su nuevo último nodo, y debe enlazar con lo que venga detrás.
---
Lleva `before`, el nodo justo delante del lote actual (al principio, un nodo ficticio). Comprueba que le siguen k nodos; inviértelos; enlaza `before.next` con el nuevo primer nodo del lote y el nuevo último nodo con el resto; después mueve `before` a ese último nodo.
