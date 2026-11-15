## Enfoque: un único registro de deshacer con marcas, y un mapa saldo → cantidad

Guarda el estado actual en dos mapas hash:

- `balance`: nombre → saldo, para `GET`;
- `holders`: saldo → cuántas cuentas lo tienen, para responder `COUNT` en O(1).

Todo cambio pasa por una única función auxiliar, `put(name, value)`: saca la cuenta de `holders` bajo su saldo anterior (si existía) y después, o bien borra la cuenta (`value` es "no existe"), o bien guarda `value` y suma uno a `holders[value]`. Tener los dos mapas detrás de una sola puerta es lo que mantiene `COUNT` correcto también después de deshacer.

**Transacciones.** En lugar de copiar el libro en cada `BEGIN`, apunta solo lo que cambia:

- `log`: una lista de pares `(name, saldo anterior o no existe)`. Mientras haya alguna transacción abierta, `SET` y `ADD` añaden el valor anterior de la cuenta antes de llamar a `put`.
- `marks`: una pila. `BEGIN` apila la longitud actual de `log`.
- `ROLLBACK` desapila una marca `m` y después saca entradas del registro mientras sea más largo que `m`, llamando a `put(name, old)` para cada una.
- `COMMIT` desapila la marca y no toca el registro. Sus entradas quedan ahora después de la marca de la transacción exterior, así que le pertenecen a ella. Cuando no queda ninguna marca, ya no se puede deshacer nada y el registro se puede vaciar.
- Con `marks` vacía, `ROLLBACK` y `COMMIT` escriben `NO TRANSACTION`.

**Ejemplo resuelto (Ejemplo 2).** `BEGIN` → `marks = [0]`. `SET cy 5` apunta `(cy, no existe)`. `BEGIN` → `marks = [0, 1]`. `SET cy 9` apunta `(cy, 5)`. `COMMIT` desapila 1; el registro conserva las dos entradas. `ROLLBACK` desapila 0 y deshace `(cy, 5)` y luego `(cy, no existe)`: `cy` desaparece, así que `GET cy` escribe `NONE`.

**Por qué es correcto.** Las entradas posteriores a una marca `m` son exactamente los cambios hechos desde ese `BEGIN`, incluidos los de transacciones internas confirmadas (una transacción interna deshecha ya quitó las suyas). Cada entrada guarda el valor justo anterior a su cambio, así que deshacerlas de la más nueva a la más antigua lleva el estado hacia atrás cambio a cambio, hasta exactamente el estado del `BEGIN`. Una cuenta cambiada varias veces tiene varias entradas, y la más antigua es la que la restaura al final. `holders` sigue siendo correcto porque cada deshacer también pasa por `put`.

**Complejidad:** O(1) esperado por orden para los mapas, más lo que se deshace. Cada cambio añade como mucho una entrada al registro, y cada entrada se deshace como mucho una vez, así que todos los `ROLLBACK` juntos cuestan O(Q). En total, tiempo y memoria O(Q).

**El enfoque evidente es demasiado lento.** Copiar el libro entero en cada `BEGIN` y recorrer todas las cuentas en cada `COUNT` cuesta O(cuentas) por orden: con 2.5·10⁴ cuentas y 8·10⁴ órdenes, muchas de ellas `BEGIN` o `COUNT`, son unos 10⁹ pasos.

**Notas prácticas:** lee toda la entrada de una vez, divídela en líneas y escribe todas las respuestas de una sola vez. `COUNT` puede preguntar por un saldo que nadie tiene, y una cuenta con saldo 0 sí cuenta para `COUNT 0` (una que no existe, no). Los saldos llegan a 10¹⁴ en valor absoluto (10⁵ · 10⁹) y pueden ser negativos: usa `i64` en Rust; un `Number` de TypeScript es exacto hasta 2⁵³ y un `Map<number, number>` sirve para `holders`; los enteros de PHP son de 64 bits. Lee `amount` con su signo (`-20`).
