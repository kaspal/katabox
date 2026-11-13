Contar los bits de cada número por separado funciona, pero la tabla puede apoyarse en sí misma. ¿Qué relación hay entre la forma binaria de `i` y la de `i / 2`?
---
Desplazar a la derecha una posición (`i >> 1`, división entera entre 2) quita el último dígito binario y conserva todos los demás.
---
Así que `bits[i] = bits[i >> 1] + (i & 1)`: los bits de `i` sin su último dígito, más ese último dígito. Rellena la tabla desde 0 hacia arriba.
