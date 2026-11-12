Deshacer quita primero la escritura más reciente, y rehacer vuelve a aplicar primero la última deshecha: las dos son "último en entrar, primero en salir".
---
Guarda los valores actuales en un mapa hash, más dos pilas. Por cada escritura, apila en la pila de deshacer lo necesario para deshacerla: la clave, el valor que sustituyó (o "ninguno") y el valor nuevo.
---
`undo` desapila de la pila de deshacer, restaura el valor antiguo y apila el mismo registro en la pila de rehacer; `redo` desapila de la pila de rehacer, vuelve a poner el valor nuevo y apila el registro otra vez en la de deshacer. Un `write` nuevo vacía la pila de rehacer.
