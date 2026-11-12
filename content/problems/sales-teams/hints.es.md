Sumar las ventas de cada empleado a todos los jefes que tiene por encima es correcto, pero en una cadena de mando larga son unas n²/2 sumas.
---
El total de un equipo son las ventas del empleado más los totales de los equipos de quienes dirige directamente. Así que, si se conoce antes el total de cada subordinado directo, el de un jefe cuesta una suma por subordinado.
---
Haz la lista de empleados de arriba abajo (un recorrido en anchura desde la dueña por las listas de hijos). Después recorre esa lista **al revés**, sumando el total de cada empleado al de su jefe: todos están terminados antes de que se use su jefe.
