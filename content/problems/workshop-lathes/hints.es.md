Simula el taller: para cada trabajo necesitas el torno que se libera antes. Recorrer todos los tornos para encontrarlo cuesta O(lathes) por trabajo.
---
Solo necesitas el menor instante "libre en" y después lo cambias. ¿Qué estructura da el mínimo rápido y admite cambios?
---
Mantén un montículo de mínimos con los instantes en que cada torno queda libre (empieza con `min(lathes, n)` ceros). Para cada trabajo, saca el menor instante `t` y mete `t + job`. La respuesta es el mayor instante metido.
