El número de salas que necesitas es el mayor número de reuniones que ocurren en el mismo momento. ¿Cuándo puede subir ese número?
---
Solo sube cuando empieza una reunión. Recorre el tiempo en orden: cada inicio ocupa una sala y cada final libera una.
---
Ordena por separado todas las horas de inicio y todas las de fin. Recórrelas con un puntero en cada lista: si el siguiente inicio es anterior al siguiente fin, se ocupa una sala; si no, se libera una (con horas iguales, primero se libera). El máximo de salas en uso es la respuesta.
