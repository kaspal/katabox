## La idea

Un **camino euleriano** recorre cada arista de un grafo exactamente una vez; los nodos se pueden visitar tantas veces como haga falta. Es el acertijo de dibujar una figura sin levantar el lápiz ni repasar una línea. Si el recorrido acaba donde empezó, es un **circuito euleriano**.

Que exista o no depende solo de los **grados** (el número de aristas de cada nodo) y de la conexión. Cada vez que el recorrido pasa por un nodo usa una arista para entrar y otra para salir, así que los nodos intermedios necesitan grado par. Solo los dos extremos pueden ser impares. Por eso un grafo no dirigido tiene camino euleriano solo si tiene **0 o 2 nodos de grado impar** y todas sus aristas están en un mismo trozo conexo: con 0 el camino es un circuito y puede empezar en cualquier parte; con 2 debe empezar en un nodo impar y acabar en el otro.

El ejemplo comprueba primero los grados y devuelve `[-1]` si el número de nodos impares no es 0 ni 2. Empieza en el primer nodo impar o, si no hay ninguno, en la `u` de la primera arista. Después aplica el **algoritmo de Hierholzer** con una **pila**. Mira el nodo de la cima: si aún tiene una arista sin usar, la marca como usada y mete el otro extremo. Si no le queda ninguna, lo saca y lo pone en el camino. Un **puntero al siguiente** por nodo dentro de su lista de aristas hace que las aristas usadas se salten una sola vez. Quedarse atascado pronto no es un error: los nodos sacados forman el final de la ruta, y los bucles encontrados después se insertan delante de ellos. Por eso el camino sale **al revés** y el ejemplo le da la vuelta al final. Si el camino tiene menos de `m + 1` nodos, algunas aristas nunca se alcanzaron (están en otro trozo del grafo) y la respuesta es `[-1]`.

### La entrada por defecto

Cinco nodos, no dirigido: `0 1, 1 2, 2 3, 3 0, 0 2, 1 4, 4 2`, un cuadrado con una diagonal y un tejado. Los grados son 3, 3, 4, 2, 2: los nodos 0 y 1 son impares, así que el recorrido empieza en 0. Siguiendo la primera arista sin usar de cada nodo, mete 0, 1, 2, 3, 0, 2, 4, 1 y usa las siete aristas sin atascarse. Al nodo 1 no le queda nada, así que sale el primero, y después 4, 2, 0, 3, 2, 1, 0. Al darle la vuelta, la salida es `[0, 1, 2, 3, 0, 2, 4, 1]`: ocho nodos, siete aristas, terminando en el otro nodo impar.

## Pasos

1. Construye listas de adyacencia que guarden el identificador de cada arista y cuenta los nodos de grado impar; si no son 0 ni 2, devuelve `[-1]`.
2. Mete el nodo de partida en la pila.
3. Mientras la pila no esté vacía, mira su cima `u`. Sáltate sus aristas usadas; si queda alguna, márcala como usada y mete su otro extremo. Si no, saca `u` y ponlo en el camino.
4. Si el camino tiene `m + 1` nodos, devuélvelo al revés; si no, devuelve `[-1]`.

## Coste

| Operación | Coste |
|---|---|
| Ejecución completa | `O(n + m)`: cada arista se usa una vez y cada puntero solo avanza |
| Espacio extra | `O(n + m)`: las listas, las marcas de uso, la pila y el camino |

## Errores comunes

- **Andar de forma voraz e ir escribiendo sobre la marcha.** Un recorrido puede volver a su inicio antes de cubrirlo todo; la pila es lo que inserta los bucles que faltan.
- **Marcar nodos en vez de aristas.** Los nodos se repiten todo el rato; son las aristas las que solo se pueden usar una vez, y por eso cada entrada de la lista lleva el identificador de la arista.

## Cuándo usarlo

Úsalo para rutas que deben cubrir cada calle o enlace una vez: quitanieves, reparto de correo, máquinas de dibujo.

## Temas relacionados

- [Pila](learn:linear/stack): la pila que guarda el recorrido actual.
- [Búsqueda en profundidad (DFS)](learn:graphs/dfs): Hierholzer es un DFS sobre aristas en vez de sobre nodos.
- [Componentes conexas](learn:graphs/connected-components): todas las aristas deben estar en una sola componente.
