Con una sola fila, es el clásico tramo de suma máxima de un arreglo. ¿Puedes convertir el problema del rectángulo en uno de una sola fila?
---
Fija las filas superior e inferior de la parcela. Sumar cada columna entre ellas da un único arreglo, y el mejor tramo de ese arreglo (algoritmo de Kadane, O(C)) es la mejor parcela con exactamente esas dos filas.
---
Recorre la fila superior; después baja la fila inferior de una en una, sumando esa fila a unas sumas por columna acumuladas, y aplica Kadane tras cada paso: O(R²·C) en total. Pon el lado más corto en los bucles exteriores: si R > C, traspón primero la cuadrícula, así un campo de 4000 × 20 cuesta unos 20²·4000 pasos en vez de 4000²·20.
