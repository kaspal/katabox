Contar los tres tipos y reescribir la cinta lleva dos pasadas. ¿Puedes mantener la cinta dividida en zonas mientras avanzas?
---
Mantén tres fronteras: todo lo anterior a `low` es vidrio, todo lo posterior a `high` es plástico y todo lo que hay entre `low` y `mid` es papel. La parte de `mid` a `high` aún está sin ordenar.
---
Mira `items[mid]`: vidrio → intercámbialo con `items[low]` y avanza `low` y `mid`; papel → avanza `mid`; plástico → intercámbialo con `items[high]` y retrocede `high` (no avances `mid`: el objeto que ha llegado aún no se ha mirado).
