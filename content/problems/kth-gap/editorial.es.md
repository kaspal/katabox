## Enfoque: búsqueda binaria sobre la respuesta, contando con dos punteros

Ordena los postes. Sea `count(d)` el número de pares cuya distancia es como mucho `d`. Nunca decrece al crecer `d`, y la k-ésima distancia es el menor `d` con `count(d) ≥ k`.

**Contar en O(n):** para cada `j`, avanza `i` mientras `posts[j] − posts[i] > d`; entonces los postes `i..j−1` están todos a distancia ≤ `d` del poste `j`, lo que suma `j − i` pares. `i` nunca retrocede, así que el recuento entero es O(n).

**Buscar:** búsqueda binaria de `d` en `[0, posts[n−1] − posts[0]]`: si `count(mid) ≥ k`, la respuesta es como mucho `mid`; si no, es mayor. Unas 20 rondas cubren 10⁶.

### Paso a paso

Postes ordenados `[2, 3, 7, 10]`, k = 4. count(4) = 3 (distancias 1, 3, 4), count(5) = 4. Así que la respuesta es 5.

**Complejidad:** O(n log n + n log W) en tiempo, donde W es la mayor distancia; O(1) de espacio extra tras ordenar. Listar todas las distancias es O(n²), más de 10⁹ pares.
