Volver a contar los identificadores distintos de los primeros `p` escaneos en cada pregunta repite casi el mismo trabajo una y otra vez: hasta 30 000 preguntas sobre hasta 30 000 escaneos es demasiado.
---
La respuesta solo depende de `p`, y nunca baja cuando `p` crece: el escaneo `p` suma un socio nuevo justo cuando su identificador no apareció en los escaneos `1 .. p-1`.
---
Recorre los escaneos una sola vez con un conjunto de los identificadores vistos y guarda `distintos[p]`, el tamaño del conjunto tras el escaneo `p`. Así cada pregunta es una sola consulta: escribe `distintos[p]`.
