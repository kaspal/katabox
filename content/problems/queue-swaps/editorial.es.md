## Enfoque: contar inversiones con el ordenamiento por mezcla

**La respuesta es el número de inversiones.** Una inversión es una pareja de posiciones `i < j` con `heights[i] > heights[j]`. Cambiar a dos vecinos solo cambia el orden relativo de esos dos niños, así que el número de inversiones cambia exactamente en uno: baja uno si eran una inversión y sube uno si no. Una fila ordenada no tiene ninguna, así que hacen falta al menos `inversiones` cambios. El ordenamiento de burbuja, que solo cambia vecinos invertidos, usa exactamente esos. Las alturas iguales nunca son una inversión y nunca necesitan un cambio.

**Contarlas en O(n log n).** El ordenamiento por mezcla parte la fila por la mitad, ordena cada mitad recursivamente y las mezcla. Una inversión está dentro de la mitad izquierda, dentro de la derecha (las cuentan las llamadas recursivas) o entre las dos mitades. Durante la mezcla, cuando el siguiente menor sale de la mitad derecha mientras aún esperan `k` elementos de la izquierda, esos `k` son todos más altos y están a su izquierda: son exactamente `k` inversiones cruzadas. Tomar de la izquierda en los empates (`<=`) evita contar alturas iguales.

```
ordenar(a):
    si len(a) <= 1: devolver a, 0
    izq, x = ordenar(a[:mitad]); der, y = ordenar(a[mitad:])
    mezclar; cada vez que der[j] < izq[i]: cuenta += len(izq) - i
    devolver mezclado, x + y + cuenta
```

### Paso a paso

`[4, 3, 2, 1]`: las mitades `[4, 3]` y `[2, 1]` tienen una inversión cada una y quedan `[3, 4]` y `[1, 2]`. Al mezclar: el 1 va antes del 3 y del 4 (+2), y el 2 también (+2). Total 1 + 1 + 4 = 6.

**Complejidad:** O(n log n) de tiempo y O(n) de memoria extra. La respuesta puede llegar a unos 5·10⁹, que necesita enteros de 64 bits. Comprobar cada pareja cuesta O(n²).
