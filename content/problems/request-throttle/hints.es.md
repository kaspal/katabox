Para un usuario, ¿cuáles de sus peticiones aceptadas pueden importar aún en el instante `t`?
---
Solo las posteriores a `t − window`. Como el tiempo solo avanza, una petición que ha salido de la ventana nunca vuelve.
---
Guarda, por usuario, una cola con los instantes de sus peticiones aceptadas. En cada operación, quita del principio los instantes `≤ time − window`; la longitud de la cola es el recuento. Una petición se acepta cuando la longitud es menor que `limit`, y entonces su instante se añade al final.
