## Enfoque: union-find con un contador de islas

El método obvio rellena una casilla y vuelve a contar las islas con un relleno por inundación sobre toda la tierra. Con 5·10⁴ casillas repartidas por un mar de 1000 × 1000, la tierra llega a 5·10⁴ casillas y los recuentos visitan unas 5·10⁴ · 5·10⁴ / 2 ≈ 1.25·10⁹ casillas en total. Demasiado lento, y un desperdicio: una casilla nueva solo afecta a sus cuatro vecinas.

**Cómo cambia la cuenta una casilla.** Si la casilla ya es tierra, no cambia nada. Si no, primero es una isla propia: `islands += 1`. Después mira sus cuatro vecinas de lado. Cada vecina que esté en una isla a la que la casilla nueva aún no se ha unido se une a ella: `islands −= 1`. Dos vecinas de la misma isla deben contar una sola vez, así que la pregunta siempre es "¿están ya estas dos casillas en la misma isla?".

**Union-find** responde a eso. Cada casilla de tierra tiene un `parent`; siguiendo los padres se llega a la raíz de la isla, y dos casillas están en la misma isla exactamente cuando tienen la misma raíz. Unir dos islas enlaza una raíz con la otra.

Ejemplo 1, cuarta casilla (1, 2): empieza como una cuarta isla (4). Su vecina (1, 1) tiene otra raíz: se unen, 3. (1, 3): otra distinta, 2. (2, 2): distinta, 1. La fila 0 de arriba es mar. La respuesta es 1.

**Disposición.** Numera la casilla de la fila `r` y la columna `c` como `r·(C + 2) + c`, y deja un borde de mar alrededor del mapa (filas 0 y `R + 1`, columnas 0 y `C + 1`). Las vecinas son entonces `cell ± 1` y `cell ± (C + 2)`, nunca fuera del mapa, sin comprobar límites. Un único array plano `parent` de `(R + 2)·(C + 2)` entradas guarda −1 para el mar y el padre para la tierra.

**Por qué es correcto:** las islas son los grupos conexos de casillas de tierra. Añadir una casilla añade un grupo y luego lo une con cada grupo distinto que toca por un lado; cada unión baja la cuenta exactamente en uno, y tocar dos veces el mismo grupo lo detecta la comparación de raíces. Las esquinas no se miran nunca, así que las casillas en diagonal siguen separadas.

**Complejidad:** O(R·C) para preparar el array, y luego como mucho cuatro búsquedas de raíz por casilla, cada una O(log(R·C)) amortizado en el peor caso acortando el camino a la mitad, y casi constante en la práctica. Memoria O(R·C), unas 10⁶ entradas.

**Notas prácticas:** lee todos los tokens de una vez y escribe las `Q` respuestas de una sola vez. Busca las raíces con un bucle (`parent[x] = parent[parent[x]]; x = parent[x]`), no con recursión: una cadena puede tener miles de casillas, más que el límite de recursión de Python. En TypeScript, un `Int32Array` guarda el millón de entradas de forma compacta. En PHP, pasa el array a una función auxiliar por referencia (`array &$parent`): pasarlo por valor lo copia en la primera escritura.
