Para una velocidad dada, ¿puedes calcular rápido el total de horas? ¿Cómo cambia ese total al subir la velocidad?
---
Una impresora más rápida nunca necesita más horas. Así que las velocidades que funcionan forman un rango: toda velocidad desde la respuesta hacia arriba funciona y ninguna más lenta.
---
Haz búsqueda binaria de la velocidad entre 1 y `max(pages)` (esa velocidad siempre funciona). Para la velocidad del medio, suma ⌈p / speed⌉; si cabe en `hours`, busca más abajo; si no, más arriba.
