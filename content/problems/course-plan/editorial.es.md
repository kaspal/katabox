## Enfoque: algoritmo de Kahn, un periodo cada vez

Piensa en los cursos como un grafo dirigido con una arista `a → b` por cada regla. Un curso se cursa en el periodo justo después de aprobar su **último** requisito, o en el periodo 1 si no tiene ninguno.

**Contadores y listas de desbloqueo.** Por cada regla `a b`, añade `b` a `unlocks[a]` y suma 1 a `missing[b]`. Una regla repetida se cuenta dos veces en ambos sitios, así que todo sigue cuadrando.

**Periodos por capas.**

1. El primer periodo son todos los cursos con `missing = 0`.
2. Para el periodo actual: ordénalo, guárdalo y, para cada uno de sus cursos `a`, baja `missing[b]` para cada `b` de `unlocks[a]`. Un curso cuyo contador baja a 0 acaba de tener aprobado su último requisito, así que pertenece al periodo **siguiente**.
3. Repite hasta que un periodo salga vacío.

**Ejemplo** (Ejemplo 1). `missing` es 0 para 1, 2 y 4: el periodo 1 es `1 2 4`. Al aprobarlos, el curso 3 queda en 0 (el 1 y el 2 están hechos), el curso 5 en 1 (espera al 3) y el curso 6 en 1 (espera al 5). El periodo 2 es `3`, que libera el 5; el periodo 3 es `5`, que libera el 6; el periodo 4 es `6`. Cuatro periodos.

**Por qué es correcto:** el contador de un curso llega a 0 justo mientras se procesa el periodo de su último requisito, así que cae en el periodo siguiente a ese, que es lo que hace la estudiante. Si las reglas contienen un ciclo, ningún curso del ciclo llega nunca a 0 (cada uno espera a otro del ciclo), ni tampoco ningún curso que dependa del ciclo. Así que, si se aprobaron menos de `n` cursos, escribe `IMPOSSIBLE`; si no hay ciclo, se llega a todos los cursos, porque entre los que faltan siempre queda alguno con contador 0.

**Complejidad:** cada curso se coloca una vez y cada regla se recorre una vez: O(n + m), más ordenar los periodos, O(n log n) en total. Memoria O(n + m).

**Por qué lo obvio es demasiado lento:** revisar todos los cursos y sus reglas al empezar cada periodo cuesta O(n + m) por periodo, y una cadena oculta de cursos puede obligar a `n` periodos de un curso cada uno. Con `n = 3·10⁴` y `m = 4·10⁴` son unos 2·10⁹ pasos.

**Notas prácticas:** lee todos los tokens de una vez y escribe el plan de una sola vez. El método usa una lista simple por periodo y nada de recursión, así que una cadena de 3·10⁴ periodos no es problema (una búsqueda en profundidad recursiva chocaría con el límite de recursión de Python). En TypeScript, ordena cada periodo con `(x, y) => x - y`, no con el orden de texto por defecto (que pone `10` antes que `9`).
