Una cola por libro resuelve `"hold"` y `"return"` en O(1). El problema es `"cancel"`: quitar a alguien del medio de una lista cuesta un tiempo proporcional a la lista.
---
No quites enseguida a quien cancela. Recuerda quién sigue esperando de verdad (un mapa hash con clave libro y socio) y, cuando vuelva un ejemplar, saca socios del principio y **sáltate** a los que ya no esperan.
---
Cuidado con quien cancela y vuelve a reservar: su entrada antigua sigue en algún lugar de la cola. Da a cada reserva un número de tique único, guárdalo en la cola y en el mapa, y acepta una entrada de la cola solo si su tique es el actual del socio.
