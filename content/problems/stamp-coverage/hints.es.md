Entintar cada casilla de cada sello puede suponer 10⁵ sellos × 2,5·10⁵ casillas. En una dimensión, ¿cómo sumas 1 a todo un tramo con dos escrituras?
---
Un array de diferencias: +1 al inicio del tramo y −1 justo después; una suma acumulada reconstruye después los recuentos. En dos dimensiones, un rectángulo necesita cuatro marcas.
---
Marca +1 en (top, left), −1 en (top, right+1), −1 en (bottom+1, left) y +1 en (bottom+1, right+1). Después haz sumas acumuladas a lo largo de cada fila y luego hacia abajo en cada columna; lee el máximo y cuántas veces aparece.
