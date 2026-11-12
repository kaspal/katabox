Recorrer cada tramo buscando su máximo y su mínimo cuesta k pasos por tramo, hasta unos 2,5·10⁹ en total. Pasar de un tramo al siguiente solo añade una lectura y quita otra.
---
Piensa solo en el máximo. Una lectura que va seguida, dentro de la ventana, de otra al menos igual de alta ya nunca puede ser el máximo: la posterior se queda más tiempo en la ventana. Así que solo importa una secuencia decreciente de candidatas.
---
Guarda las candidatas a máximo en una cola doble de índices con valores decrecientes: antes de añadir una lectura, saca por detrás las menores; quita la de delante cuando sale de la ventana. La de delante es el máximo. Usa una segunda cola (creciente) para el mínimo.
