Piensa primero en el encargo mejor pagado: ¿hay alguna razón para rechazarlo? Después en el siguiente, y así sucesivamente. La pregunta es qué día dar a cada encargo que te quedas.
---
Toma los encargos de mayores a menores honorarios y pon cada uno en el día libre **más tardío** dentro de su plazo, y sáltalo cuando todos los días hasta su plazo estén ocupados. A los días tardíos les sirven menos encargos, así que esto deja libres los días tempranos para los encargos de plazo corto. Un plazo mayor que `n` se puede tratar como `n`: ningún plan usa más de `n` días.
---
Buscar hacia atrás día a día un día libre es demasiado lento cuando muchos encargos comparten plazo. Guarda `free[d]`, un enlace hacia el día libre más tardío ≤ `d`, donde el día 0 significa "no queda ninguno" (union-find sobre los días `0..n`). Sigue los enlaces (acortándolos sobre la marcha) para encontrar el día; cuando ocupes el día `d`, pon `free[d] = d − 1`.
