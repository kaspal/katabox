Convierte cada `hh:mm` en minutos desde medianoche (`hh · 60 + mm`) y trabaja con números normales; vuelve a `hh:mm` solo al escribir la respuesta.
---
Ordena los tramos del día por hora de inicio. Al recorrerlos en ese orden, lleva `free_from`: el primer minuto que no cubre ningún tramo visto hasta ahora (empieza en `open`). Un tramo que empieza después de `free_from` deja un hueco libre justo antes.
---
Para cada tramo `(s, e)` en orden de inicio: si `s − free_from` es mayor que el mejor hasta ahora, guarda el hueco `free_from..s`; después haz `free_from = max(free_from, e)`. Tras el último tramo, comprueba también el hueco `free_from..close`. Cambia el mejor solo cuando un hueco sea **estrictamente** más largo, para que en los empates gane el más temprano. Una longitud mejor de 0 significa `BOOKED`.
