Piensa en los pasos como nodos y en cada regla `[a, b]` como una flecha `a → b`. Un paso está listo cuando todas las flechas que llegan a él salen de pasos terminados.
---
Cuenta, para cada paso, cuántos pasos sin terminar está esperando. Cuando se termina un paso, baja la cuenta de cada paso al que apunta; los que llegan a cero quedan listos.
---
Guarda los pasos listos en un montículo de mínimos para que salga primero el número más bajo (algoritmo de Kahn). Si el montículo se vacía antes de hacer los `n` pasos, hay un ciclo: devuelve `[]`.
