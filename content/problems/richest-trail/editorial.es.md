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

`[6, -8, 5, 3, null, 4, -2]`: ganancia(3) = 3, ganancia(−8) = −8 + 3 = −5, ganancia(4) = 4, ganancia(−2) = −2, ganancia(5) = 5 + 4 = 9. El doblez en 5 vale 5 + 4 + 0 = 9; el doblez en 6 vale 6 + max(0, −5) + max(0, 9) = 15. La respuesta es 15.

**Complejidad:** O(n) en tiempo y O(h) para la recursión.
