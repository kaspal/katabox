## Enfoque: ordenar, sumas de prefijos, búsqueda binaria

**Primero los más baratos.** Si un equipo puede pagar `k` paquetes cualesquiera, también puede pagar los `k` más baratos: cambiar un paquete elegido por otro más barato no elegido nunca sube el coste. Así que el máximo de paquetes que compra un presupuesto es el mayor `k` tal que los `k` precios más baratos suman como mucho el presupuesto.

**Preparar una vez.** Las existencias son las mismas para todos los equipos, así que ordena los precios una vez y construye los totales acumulados:

```
total[0] = p[0],   total[i] = total[i − 1] + p[i]     (p ordenado, del más barato al más caro)
```

`total[k − 1]` es el coste de los `k` paquetes más baratos. Los precios son positivos, así que los totales crecen estrictamente, y la respuesta para un presupuesto es **el número de totales que no superan el presupuesto**. Como la lista está ordenada, una búsqueda binaria halla ese número en O(log n).

**Ejemplo** (Ejemplo 1). Precios ordenados `3 3 5 7 12`, totales `3 6 11 18 30`. Presupuesto 10: caben dos totales (3 y 6), así que 2. Presupuesto 18: cuatro totales, porque el propio 18 cuenta (`≤`, no `<`). Presupuesto 2: ninguno, así que 0. Presupuesto 30: los cinco.

**Por qué es correcto:** el argumento de intercambio muestra que los `k` paquetes más baratos son la forma más barata de conseguir `k` paquetes; los totales son crecientes, así que los valores de `k` que caben forman un prefijo de la lista, y contar los totales dentro del presupuesto da su longitud.

**Complejidad:** O(n log n) para ordenar y O(n) para los totales, una sola vez; O(log n) por presupuesto. O((n + Q) log n) en total, con memoria O(n).

**Por qué lo obvio es demasiado lento:** recorrer los precios ordenados para cada presupuesto hasta que se acabe el dinero es O(n) por equipo. Con 5·10⁴ paquetes y 5·10⁴ equipos cuyos presupuestos alcanzan casi todo el total, son hasta 2.5·10⁹ pasos.

**Notas prácticas:** lee todos los tokens de una vez y escribe todas las respuestas de una sola vez. Los totales llegan a 5·10⁴ · 10⁹ = 5·10¹³, más de 32 bits: usa `u64` o `i64` en Rust. En TypeScript un `Number` los guarda exactamente (quedan por debajo de 2⁵³), y lo mismo un entero de PHP en una versión de 64 bits. En TypeScript, ordena números con `(a, b) => a - b`: el `sort()` por defecto los compara como texto. Busca el primer total **mayor** que el presupuesto (una cota superior); su posición es la respuesta.
