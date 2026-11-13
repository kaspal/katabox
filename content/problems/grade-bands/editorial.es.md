## Enfoque: una cadena if / else-if por nota

Recorre las notas en orden y añade una letra por cada una. Comprobar los umbrales de mayor a menor hace que cada prueba solo necesite un límite inferior:

- `nota >= 90` → `"A"`
- si no, `nota >= 80` → `"B"`
- si no, `nota >= 70` → `"C"`
- si no, `nota >= 60` → `"D"`
- si no, `"F"`

Cuando se evalúa `nota >= 80`, ya se sabe que la nota es menor que 90, así que no hace falta un límite superior.

**Complejidad:** O(n) en tiempo, O(n) para el resultado.
