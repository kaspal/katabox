## Enfoque: ordenar, fijar un regalo y barrer los otros dos

Ordena los precios. Cualquier trío, escrito en orden, tiene su regalo más barato en algún índice `i` y los otros dos en índices `lo < hi` posteriores. Para un `i` fijo:

- empieza con `lo = i + 1` y `hi = n − 1`;
- si `s = p[i] + p[lo] + p[hi]` queda por debajo del objetivo, toda pareja con este `lo` y un `hi` menor queda aún más abajo, así que mueve `lo` a la derecha;
- si `s` queda por encima, por el mismo argumento mueve `hi` a la izquierda;
- si `s` es igual al objetivo, esa es la respuesta.

Cada barrido mira todos los candidatos que podrían ser el mejor para ese `i`, así que quedarse con el mejor `s` entre todos los `i` da la respuesta. "Mejor" significa un `|s − target|` menor y, en empate, el `s` menor.

```
ordenar p; mejor = p[0] + p[1] + p[2]
para i en 0 .. n-3:
    lo, hi = i + 1, n - 1
    mientras lo < hi:
        s = p[i] + p[lo] + p[hi]
        si mejora(s, mejor): mejor = s
        si s < target: lo += 1
        si no, si s > target: hi -= 1
        si no: devolver s
devolver mejor
```

### Paso a paso

`prices = [10, 20, 30, 40]`, `target = 75`. Con `i = 0` el barrido ve 70 (por debajo: `lo` avanza) y después 80 (por encima: `hi` retrocede) y termina. Los dos están a 5, así que se queda 70. Con `i = 1` ve 90, más lejos. Respuesta 70.

**Complejidad:** O(n²) de tiempo tras ordenar en O(n log n), y O(1) de espacio adicional. Revisar todos los tríos es O(n³).
