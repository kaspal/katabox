## Enfoque: mejor ganancia hacia abajo y mejor doblez en cada nodo

Toda ruta tiene una única cueva más alta `v`; desde ahí puede bajar al subárbol izquierdo, al derecho, a los dos o a ninguno.

Define `ganancia(v)` = el mejor total de una ruta descendente que empieza en `v`:

```
ganancia(v) = val(v) + max(0, ganancia(izq), ganancia(der))
```

(un hijo que falta aporta 0; una rama negativa simplemente no se toma). La mejor ruta que se dobla en `v` es

```
val(v) + max(0, ganancia(izq)) + max(0, ganancia(der))
```

Un recorrido en postorden calcula `ganancia` para cada nodo y se queda con el mejor doblez. La respuesta empieza en −∞: con todos los valores negativos, la mejor ruta es la cueva menos negativa.

### Paso a paso

`[-5, 4, 8, null, null, -2, 6]`: ganancia(−2) = −2, ganancia(6) = 6, ganancia(8) = 8 + 6 = 14, ganancia(4) = 4. El doblez en 8 vale 8 + 0 + 6 = 14; el doblez en −5 vale −5 + 4 + 14 = 13. La respuesta es 14.

**Complejidad:** O(n) en tiempo y O(h) para la recursión.
