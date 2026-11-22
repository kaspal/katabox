En el muro, el más bajo de los dos lados fijaba el nivel del agua. En una cuadrícula, el agua se escapa por el punto **más bajo** de lo que la rodea. ¿Dónde está el punto más bajo del anillo exterior?
---
Mira la columna más baja del borde. Su vecina sin visitar no puede guardar agua por encima de esa altura, porque el agua se saldría justo por ahí. ¿Puedes hacer crecer la zona conocida hacia dentro, siempre desde su punto más bajo del contorno?
---
Mete todas las columnas del borde en un montículo de mínimos ordenado por nivel (su altura). Saca la más baja; por cada vecina sin visitar, suma `max(0, nivel − altura)` de agua y métela con nivel `max(nivel, altura)`. Cada columna se visita una vez.
