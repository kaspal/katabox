## La idea

Una barra de longitud `n` se puede vender entera o cortada en trozos, y un trozo de longitud `k` se vende por `prices[k - 1]`. Los cortes son gratis. ¿Qué cortes dan más dinero? La longitud de la barra es la cantidad de precios dados.

Piensa en el **primer trozo** que se corta por el extremo izquierdo. Si mide `cut`, gana su precio, y el resto, de longitud `L - cut`, debería cortarse también de la mejor manera, que es una versión más pequeña de la misma pregunta. Así que:

`best[L] = máximo, para cut de 1 a L, de prices[cut - 1] + best[L - cut]`

con `best[0] = 0`. Tomar `cut = L` significa vender el trozo entero. El ejemplo llena `best` desde la longitud 1 hacia arriba y **mejora** una celda solo cuando un corte es estrictamente mejor, así que en caso de empate gana el primer trozo **más corto**. Además recuerda ese trozo en `first[L]`. Después la reconstrucción recorre la barra: toma `first[n]`, pasa al resto, toma su primer trozo, y así hasta que no queda nada. La salida es `best[n]` y después las longitudes de los trozos en ese orden.

Esta es la **mochila sin límite**: las longitudes de los trozos son los objetos, la longitud de la barra es la capacidad, y un objeto se puede usar **cuantas veces se quiera**. En la mochila 0/1 cada objeto tiene su propia fila y se usa como mucho una vez; aquí `best[L - cut]` puede contener ya un trozo de longitud `cut`, y eso está permitido. Basta una fila de celdas.

### La entrada por defecto

Precios `2, 5, 7, 8, 12, 13` para las longitudes 1 a 6. `best[1] = 2`. `best[2] = 5`: el trozo entero gana a 2 + 2. `best[3] = 7`: 2 + 5 llega primero; 5 + 2 y el trozo entero solo empatan. `best[4] = 10` con 5 + 5. `best[5] = 12`: 2 + 10 llega primero, y 5 + 7 y el trozo entero solo empatan. Para la longitud 6 los primeros trozos ofrecen 14, **15**, 14, 13, 14 y 13, así que `best[6] = 15` con un primer trozo de 2. La tabla es `[0, 2, 5, 7, 10, 12, 15]`.

Reconstrucción: 6 empieza con 2, los 4 restantes empiezan con 2 y los 2 restantes son un trozo de 2. La salida es `[15, 2, 2, 2]`: tres trozos de longitud 2, frente a 13 por la barra entera. La mochila 0/1 no podría usar el mismo trozo tres veces.

## Pasos

1. Pon `best[0] = 0`.
2. Para cada longitud `L` de 1 a `n`, y cada primer trozo `cut` de 1 a `L`: si `prices[cut - 1] + best[L - cut] > best[L]`, guárdalo y pon `first[L] = cut`.
3. Reconstrucción: desde `L = n`, apunta `first[L]` y réstalo, hasta que `L = 0`.
4. Devuelve `best[n]` y los trozos apuntados.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n²)`: cada longitud prueba todos los primeros trozos |
| Espacio extra | `O(n)`: `best` y `first` |

## Errores comunes

- **Desfase de uno en los precios.** Un trozo de longitud `cut` se vende por `prices[cut - 1]`.
- **Olvidar la barra entera.** El bucle debe incluir `cut = L`, o una barra que se vende mejor sin cortar se troceará siempre.
- **No guardar la elección.** `best` por sí solo da el dinero, no los cortes; guarda `first`.

## Cuándo usarlo

Usa el patrón sin límite siempre que las piezas se puedan repetir: cortar material en stock, llenar un presupuesto con compras repetibles o maximizar el valor con copias ilimitadas de cada objeto.

## Temas relacionados

- [Mochila 0/1](learn:dynamic-programming/knapsack-01): la versión en la que cada objeto se usa como mucho una vez.
- [Cambio de monedas](learn:dynamic-programming/coin-change): otra vez piezas reutilizables, minimizando una cuenta.
- [Subir escaleras](learn:dynamic-programming/climbing-stairs): otra regla de "primer trozo y después el resto".
- Multiplicación de cadenas de matrices (próximamente): elegir dónde partir, sobre intervalos.
