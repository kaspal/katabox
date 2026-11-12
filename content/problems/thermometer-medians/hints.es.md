Volver a ordenar todas las lecturas tras cada nueva cuesta O(n² log n). La mediana solo necesita el valor o los dos valores del centro.
---
Separa las lecturas en una mitad baja y una mitad alta, donde todo valor de la baja es ≤ que todo valor de la alta. La mediana sale del mayor valor de la mitad baja y del menor de la alta.
---
Guarda la mitad baja en un montículo de máximos y la alta en uno de mínimos, con tamaños iguales o la baja una más grande. Mete la lectura nueva en el montículo que toca y pasa un valor de uno a otro si los tamaños se descompensan. Cada paso es O(log n).
