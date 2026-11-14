## La idea

Un mensaje usa unos pocos símbolos, algunos mucho más a menudo que otros. Un **código de longitud fija** da a cada símbolo el mismo número de bits: seis símbolos necesitan 3 bits cada uno. Un **código de longitud variable** da códigos cortos a los símbolos frecuentes y largos a los raros, así que el mensaje entero se acorta.

Las longitudes variables plantean una pregunta: ¿dónde acaba un código y empieza el siguiente? La respuesta es un **código libre de prefijos**, en el que ningún código es el comienzo de otro. Coloca los códigos en un árbol binario, con 0 en una arista y 1 en la otra, y los símbolos solo en las hojas; cada código es el camino de la raíz a su hoja, y leer bits desde la raíz siempre se detiene en exactamente un símbolo. La longitud del código de un símbolo es la **profundidad** de su hoja.

El **algoritmo de Huffman** construye el árbol desde abajo. Cada símbolo empieza como un árbol de una hoja con su frecuencia como peso. Una y otra vez, toma los **dos árboles más ligeros** y únelos bajo un nuevo padre cuyo peso es su suma. Cuando queda un solo árbol, ese es el código. Los empates van para el árbol **más antiguo**: primero los símbolos originales, en el orden de la entrada, y después los árboles unidos en el orden en que se crearon. La salida es el número total de bits del mensaje y después la longitud del código de cada símbolo. La aplicación llama a los símbolos a, b, c… en el orden de la entrada.

¿Por qué los dos más ligeros? Los dos símbolos más raros siempre pueden ir juntos en el nivel más profundo de algún árbol óptimo: si un símbolo más frecuente estuviera más abajo, cambiarlo por uno más raro no añadiría bits. Unirlos deja el mismo problema con un símbolo menos, cuyo peso es su suma.

### La entrada por defecto

Frecuencias `4, 7, 11, 15, 18, 40` para a a f. Se unen a y b en 11. Ahora hay dos árboles que pesan 11; c es más antiguo, así que c y el nuevo árbol se unen en 22. Después d y e forman 33, luego 22 y 33 forman 55, y al final f y 55 forman la raíz, 95.

La salida es `[216, 4, 4, 3, 3, 3, 1]`: f, el más frecuente, recibe un código de 1 bit; a y b reciben 4 bits. Un código fijo de 3 bits usaría `95 · 3 = 285` bits. El total también es igual a la suma de los pesos de todas las uniones: `11 + 22 + 33 + 55 + 95 = 216`, porque cada unión añade un bit a cada símbolo que queda debajo.

## Pasos

1. Mete cada símbolo en una cola de prioridad como `(frecuencia, id)`, con ids de 0 a `n - 1`.
2. Saca los dos más pequeños, dales un nuevo padre con el siguiente id y la suma de sus pesos, y mételo.
3. Repite hasta que quede un solo árbol.
4. La longitud del código de cada símbolo es el número de enlaces a su padre hasta la raíz.
5. Devuelve la suma de `frecuencia · longitud` y después las longitudes.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log n)`: `n - 1` uniones, cada una con unas pocas operaciones de montículo |
| Espacio extra | `O(n)` para el montículo y los enlaces a los padres |

## Errores comunes

- **Unir los árboles más pesados.** Eso hunde los símbolos frecuentes, justo lo contrario de lo que se busca.
- **Empates.** Reglas de desempate distintas pueden dar árboles y longitudes distintos, pero el total de bits es siempre el mismo. El emisor y el receptor deben construir el mismo árbol, así que fija la regla.
- **Un solo símbolo.** Un único símbolo recibiría un código de 0 bits; los codificadores reales le dan 1 bit.

## Cuándo usarlo

Los códigos de Huffman están dentro de compresores de archivos y formatos de imagen, normalmente después de otro paso que ha hecho muy comunes algunos símbolos. La misma regla de "unir los dos más pequeños" encuentra también el orden más barato para mezclar muchos archivos ordenados.

## Temas relacionados

- [Montículo binario y cola de prioridad](learn:trees/binary-heap): la cola que entrega los dos árboles más ligeros.
- [Árbol de expansión mínima de Kruskal](learn:graphs/kruskal): otro algoritmo voraz que une piezas, de la más barata a la más cara.
- [Mochila fraccionaria](learn:greedy/fractional-knapsack): una regla voraz respaldada por un argumento de intercambio.
