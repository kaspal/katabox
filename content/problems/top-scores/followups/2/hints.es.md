Ordenar cada ventana cuesta O(n · w log w), demasiado lento cuando ambos son grandes. Las
ventanas se solapan casi por completo: de una a la siguiente sale una partida y entra
otra.
---
Un montículo lo tiene difícil aquí porque las partidas viejas deben salir desde el medio.
Mira el rango de las puntuaciones: solo hay 301 valores posibles.
---
Lleva la cuenta de cuántas partidas de la ventana tienen cada puntuación. Desliza la
ventana añadiendo la partida nueva y quitando la vieja; luego recorre los valores desde
300 hacia abajo, sumando cuentas hasta llegar a `k`. Un árbol de Fenwick sobre los valores
hace ese recorrido logarítmico.
