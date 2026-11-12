## Enfoque: un montículo de mínimos con los frentes de las colas

El siguiente cliente de la cola unida es el menor turno entre los frentes actuales de las colas. Un montículo de mínimos guarda esos frentes:

1. Mete `(turno, índice de cola)` del frente de cada cola no vacía.
2. Saca el menor; añade ese nodo al resultado y avanza su cola.
3. Si a la cola le quedan clientes, mete su nuevo frente.
4. Para cuando el montículo esté vacío.

El índice de la cola desempata, así que el montículo nunca compara los nodos en sí. El resultado reutiliza los nodos existentes y los vuelve a enlazar en orden.

### Paso a paso

`lines = [[2, 5, 9], [1, 5], [3]]`: el montículo empieza con 2, 1 y 3. Sale el 1 (cola 1, entra el 5), sale el 2 (cola 0, entra el 5), sale el 3 (la cola 2 queda vacía), sale un 5, sale el otro 5 (entra el 9), sale el 9: `[1, 2, 3, 5, 5, 9]`.

**Complejidad:** O(N log k) en tiempo para N clientes en k colas, y O(k) de espacio extra para el montículo. Revisar el frente de todas las colas por cada cliente es O(N·k): con 10⁴ colas y 10⁵ clientes son 10⁹ comparaciones.
