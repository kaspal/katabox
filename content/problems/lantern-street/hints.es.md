Para cada farola hay dos situaciones: está encendida o no lo está. ¿Qué permite cada situación a sus hijos?
---
Calcula dos números por nodo, de abajo arriba: `lit(v)` = mejor total en el subárbol de v con v encendida, y `dark(v)` = mejor total con v apagada.
---
`lit(v) = val + dark(izq) + dark(der)` y `dark(v) = max(lit(izq), dark(izq)) + max(lit(der), dark(der))`. La respuesta es `max(lit(raíz), dark(raíz))`. Recalcular los nietos desde cero en su lugar es exponencial.
