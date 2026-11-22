Compara una entrada cuya suma se divide exactamente entre su longitud con otra que no, como `[1, 2]`. ¿Cuál sale mal?
---
La suma está bien. Mira con atención cómo se divide entre la cantidad.
---
`//` es la división entera en Python (y `Math.floor` en la versión de TypeScript), así que se pierde la parte decimal. Usa la división real `/` y quita el redondeo hacia abajo.
