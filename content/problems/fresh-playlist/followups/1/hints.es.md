Repetir la ventana del problema base dentro de cada petición cuesta hasta 10⁵ pasos por
petición. Mejor pasa la ventana una sola vez por todo el registro: para cada emisión `r`,
¿dónde empieza el tramo sin repeticiones más largo que termina en `r`? Llámalo `start[r]`.
---
`start[r]` nunca baja. En una petición `[a, b]`, las emisiones `r` con `start[r] < a` quedan
cortadas en `a`: su mejor tramo dentro es `a..r`, el más largo para la última de esas `r`.
Las demás conservan su tramo entero, de longitud `r − start[r] + 1`.
---
Busca con búsqueda binaria la primera `r0` de `[a, b]` con `start[r0] ≥ a`. La respuesta es
el mayor entre `r0 − a` (el tramo `a..r0 − 1`) y el máximo de `r − start[r] + 1` en
`r0..b`, que una tabla dispersa da en O(1).
