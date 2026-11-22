La estructura union-find del problema base te dice qué clubes se fusionan. ¿Qué número extra por club respondería a esta pregunta?
---
Guarda el tamaño de cada club en su raíz y suma los dos tamaños cuando se unen dos raíces. ¿Puede encogerse alguna vez el club más grande?
---
Los clubes solo crecen, y solo el club recién formado puede pasar a ser el más grande. Lleva un máximo acumulado, que empieza en 1, y compáralo con el tamaño del club nuevo después de cada fusión real; una fusión dentro de un mismo club deja la respuesta como estaba.
