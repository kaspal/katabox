Probar cada `lo` posible y buscar el turno más cercano en cada cola cuesta
`clientes × colas`, hasta 10⁹. Piensa en la mezcla del problema base: recorre todos los
turnos en orden creciente, un frente de cola cada vez.
---
En cualquier momento de la mezcla, los frentes de las colas (uno por cola) forman un rango
válido: del frente más pequeño al más grande. ¿Qué frente puedes avanzar sin perder la
ocasión de un rango más estrecho?
---
Guarda los frentes en un montículo de mínimos y lleva la cuenta del frente más grande. El
rango es `[mínimo del montículo, más grande]`. Apúntalo cuando sea estrictamente más
estrecho que el mejor hasta ahora y avanza la cola que tiene el mínimo (mete su siguiente
nodo y actualiza el más grande). Para cuando esa cola se acabe: a partir de ahí ningún
rango la cubre.
