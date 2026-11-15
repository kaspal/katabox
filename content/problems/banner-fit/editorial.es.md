## Enfoque: alturas por fila y el rectángulo más grande bajo un perfil

Todo cartel tiene su borde inferior en alguna fila. Fija esa fila y, para cada columna `x`, sea `height[x]` el número de casillas libres de la racha sin interrupciones que termina en esta fila en la columna `x`. Un cartel con el borde inferior aquí, que ocupe las columnas `left..right`, puede tener como mucho `min(height[left..right])` de alto. Así que el mejor cartel en esta fila es el **rectángulo más grande bajo el perfil** de las alturas, y la respuesta es el mejor de todas las filas.

Las alturas pasan de una fila a la siguiente en O(C): una casilla libre suma 1 y una bloqueada las pone a 0. En el Ejemplo 1:

```
.#...   1 0 1 1 1
.....   2 1 2 2 2
##...   0 0 3 3 3   → 3 barras de altura 3: área 9
.#..#   1 0 4 4 0
```

**El perfil con una pila.** Para cada barra, el rectángulo más ancho de exactamente su altura va desde justo después de la barra más baja más cercana por la izquierda hasta justo antes de la barra más baja más cercana por la derecha. Recorre las columnas de izquierda a derecha con una pila de columnas de altura creciente, y añade al final una columna de altura 0:

1. Antes de apilar la columna `x`, saca todas las columnas cuya altura sea al menos `height[x]`.
2. Una columna sacada de altura `h` llega por la derecha hasta `x − 1` (la columna `x` es la primera más baja o igual) y por la izquierda hasta justo después de la columna que queda en la cima de la pila (todas las de en medio eran más altas y ya se sacaron), o hasta la columna 0 si la pila está vacía. Su área es `h · (x − left)`.
3. Apila `x`.

**Por qué es correcto:** el rectángulo más grande bajo el perfil tiene la altura de su barra más baja y no puede pasar de una barra más baja por ninguno de los dos lados; así que es el rectángulo más ancho de la altura de alguna barra, y el recorrido calcula esa anchura para cada barra. Con alturas iguales, la primera de dos barras iguales sale antes y con una anchura menor, pero la última de la racha sale más tarde y recibe la anchura completa, así que no se pierde ningún área. Cada columna se apila y se saca una vez, así que una fila cuesta O(C).

**El enfoque obvio es demasiado lento.** Probar cada borde izquierdo y ensanchar hacia la derecha llevando la barra más baja cuesta O(C²) por fila, O(R · C²) en total: en una pared de 1500 × 1500 con solo unas decenas de casillas bloqueadas, casi cada fila es una única racha libre y son unos 1.7·10⁹ pasos. Comprobar cada rectángulo casilla por casilla es mucho peor. La pila deja toda la pared en O(R · C), unos 10⁶ pasos.

**Complejidad:** tiempo O(R · C) y memoria O(C) además de la entrada.

**Notas prácticas:** lee toda la entrada de una vez y sepárala por espacios en blanco; las filas no tienen espacios, así que cada fila es un token. En TypeScript y Rust, compara códigos de carácter (`charCodeAt`, `as_bytes()`) en vez de construir cadenas de un carácter. El área es como mucho 10⁶, así que cualquier tipo entero basta.
