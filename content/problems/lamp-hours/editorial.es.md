## Enfoque: ordenar por inicio y unir

Tras ordenar las lámparas por hora de inicio, las lámparas cuya luz se solapa o se toca quedan una junto a otra. Recórrelas guardando el tramo iluminado actual `[s, e)`:

```
ordenar lamps por inicio
s, e = lamps[0]
total = 0
para start, end en lamps[1:]:
    si start <= e:            # se solapa o toca el tramo: alárgalo
        e = max(e, end)
    si no:                    # un hueco oscuro: cierra el tramo
        total += e - s
        s, e = start, end
total += e - s
```

Cada segundo iluminado pertenece a exactamente un tramo cerrado, así que nada se cuenta dos veces. Las lámparas que se tocan (`start == e`) se unen, lo que no cambia el total pero mantiene los tramos máximos.

### Paso a paso

`[[1, 4], [2, 6], [8, 10]]` ya está ordenada. `[1, 4)` y `[2, 6)` se unen en `[1, 6)`; `[8, 10)` empieza después de 6, así que se suma 5 y el tramo nuevo suma 2. Total 7.

**Complejidad:** O(n log n) para ordenar y O(n) para el recorrido. Marcar cada segundo cuesta O(n + T) con T hasta 10⁹.
