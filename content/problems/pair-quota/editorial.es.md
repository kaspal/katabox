## Enfoque: ordenar y usar dos punteros desde los extremos

Comprobar todas las parejas cuesta `n(n − 1)/2` pasos por caso. La entrada puede ser un único caso de 10⁵ equipos: son unos 5·10⁹ parejas, demasiadas para el límite de tiempo en cualquier lenguaje. Las pruebas con muchos casos pequeños no le suponen ningún problema; el caso grande, sí.

**Primero, ordenar.** Qué parejas caben no depende del orden de los saldos, así que ordénalos: `a[0] ≤ a[1] ≤ … ≤ a[n−1]`. Ahora mira el menor y el mayor de los que siguen en juego, `a[i]` y `a[j]`:

- si `a[i] + a[j] ≤ K`, entonces `a[i]` cabe con **todos** los `a[i+1..j]`, porque cada uno es como mucho `a[j]`: son `j − i` parejas. Cuéntalas y descarta `a[i]` (`i += 1`);
- si no, `a[j]` no cabe ni siquiera con el menor de los que quedan, así que no cabe con ninguno: descarta `a[j]` (`j -= 1`).

Para cuando `i` alcance a `j`. Para el primer caso del Ejemplo 1, ordenado `2 3 5 8` con `K = 10`:

```
i=0 j=3   2 + 8 = 10 ≤ 10   cuenta += 3   (2 con 3, 5, 8)
i=1 j=3   3 + 8 = 11 > 10   descarta 8
i=1 j=2   3 + 5 = 8  ≤ 10   cuenta += 1   (3 con 5)
i=2 j=2   fin                cuenta = 4
```

**Por qué es correcto:** cada paso descarta un saldo después de resolver todas sus parejas con los saldos que siguen en juego, así que cada pareja se decide exactamente una vez, en el momento en que se descarta el primero de sus dos miembros. Las parejas con saldos descartados antes ya se resolvieron en aquel paso. Las dos reglas de arriba muestran que las parejas resueltas en cada paso se cuentan bien.

**Complejidad:** O(n log n) para ordenar y O(n) para los punteros, por caso. Como la suma de `n` en todos los casos es como mucho 10⁵, el total es O(Σn · log Σn), se repartan como se repartan los equipos entre los casos.

**Varios casos de prueba:** lee todos los tokens de una vez y recórrelos con una posición: `T`, y luego para cada caso `n`, `K` y los `n` saldos siguientes. Empieza cada caso de cero (el recuento a 0, solo los saldos de ese caso) y no reserves un array de 10⁵ en cada caso: con 10⁴ casos eso solo ya serían 10⁹ celdas. Junta las respuestas y escríbelas de una vez.

**Trampas con enteros:** el recuento puede llegar a unos 5·10⁹, más de lo que cabe en un entero de 32 bits: usa `i64` o `u64` en Rust. La suma de dos saldos llega a ±2·10⁹, que cabe justo en 32 bits; `i64` en todo es la opción segura. En TypeScript, `Number` es exacto para todos estos valores, pero `Array.prototype.sort()` sin comparador ordena **como cadenas** (`-1` antes que `-10`, `10` antes que `9`): pasa `(x, y) => x - y` o usa un `Float64Array`, cuyo `sort()` es numérico. En PHP, convierte los tokens con `intval` (o `(int)`) antes de llamar a `sort`.
