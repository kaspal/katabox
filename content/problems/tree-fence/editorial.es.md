## Enfoque: cadena monótona de Andrew y después la fórmula del lazo

1. Ordena los árboles por x y después por y (los duplicados quedan juntos).
2. **Envolvente inferior:** de izquierda a derecha. Antes de añadir un árbol, quita la última esquina mientras las dos últimas esquinas y el árbol nuevo no formen un giro estricto a la izquierda: `cross(h[−2], h[−1], p) ≤ 0`. Después añade el árbol.
3. **Envolvente superior:** lo mismo, de derecha a izquierda.
4. Únelas (quitando el último punto de cada mitad, que es donde empieza la otra). Las esquinas quedan en orden antihorario y sin puntos colineales.
5. **El doble del área** es `Σ (x_i · y_{i+1} − x_{i+1} · y_i)` alrededor de la envolvente (fórmula del lazo). Con menos de 3 esquinas, la respuesta es 0.

Usar `≤ 0` al quitar elimina los puntos colineales y duplicados, así que nunca llegan a ser esquinas.

### Por qué funciona

En la envolvente inferior, las aristas consecutivas siempre giran a la izquierda. Si añadir un árbol nuevo hiciera girar la cadena a la derecha o seguir recta, la última esquina queda sobre o por encima del segmento que une a sus vecinas, así que no es un punto extremo.

**Complejidad:** O(n log n) por la ordenación y después O(n): cada árbol entra y sale como mucho una vez por mitad. El método del envoltorio de regalo (desde cada esquina, recorrer todos los árboles buscando la siguiente) es O(n·h), que es O(n²) cuando casi todos los árboles son esquinas. Las coordenadas de hasta 10⁷ mantienen cada producto cruzado y el área por debajo de 2⁵³.
