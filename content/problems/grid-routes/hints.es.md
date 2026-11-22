Prueba un suelo sin ninguna estantería, como `floor = ["..", "..", ".."]`. Cuenta las rutas a mano y compara.
---
Sin la memoria la recursión sería correcta, solo lenta. Mira qué usa la memoria como clave: ¿pueden dos celdas distintas compartirla?
---
La memoria usa la clave `r + c`, así que todas las celdas de una misma diagonal comparten una entrada, y el primer valor guardado se reutiliza para celdas con otro número de rutas. Usa como clave la propia celda, `(r, c)` (en TypeScript, por ejemplo `r * cols + c`).
