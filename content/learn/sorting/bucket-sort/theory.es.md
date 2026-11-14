## La idea

La ordenación por cubetas reparte los valores entre unas pocas **cubetas** que cubren tramos iguales, ordena cada cubeta pequeña y lee las cubetas en orden. Aquí los valores van de 1 a 99 y hay cinco cubetas de 20: 0–19, 20–39, 40–59, 60–79 y 80–99. Cada valor de una cubeta es menor que cualquier valor de la siguiente, así que, una vez ordenada cada cubeta, leerlas de izquierda a derecha da un arreglo ordenado. Cuando los valores están bien repartidos, cada cubeta tiene solo unos pocos, y ordenar unos pocos sale barato.

## Pasos

1. Crea `count = 5` cubetas vacías.
2. Reparte: pon cada valor `v` en la cubeta `v * count // 100`. Por ejemplo, 43 va a la cubeta 2, el tramo 40–59.
3. Ordena cada cubeta. El ejemplo usa la ordenación del propio lenguaje, que va bien con cubetas diminutas; los libros suelen usar aquí la ordenación por inserción.
4. Recoge: recorre las cubetas en orden y escribe sus valores desde la posición 0.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n + k)` (`k` = número de cubetas) |
| Caso medio | `O(n + k)` con valores bien repartidos |
| Peor caso | `O(n²)`: todos los valores en una cubeta, ordenada por inserción |
| Espacio extra | `O(n + k)` |
| Estable | Sí, si la ordenación dentro de cada cubeta es estable |
| En el sitio | No |

## Errores comunes

- **Valores agrupados.** Si la mayoría de los valores cae en un mismo tramo, una cubeta se lleva casi todo el trabajo y se pierde la ventaja: el total cuesta lo que cueste ordenar esa cubeta. Con inserción dentro de las cubetas es `O(n²)`; con la ordenación del lenguaje que usa el ejemplo, `O(n log n)`. Prueba tu propia entrada con todos los valores entre 20 y 39.
- **La fórmula de la cubeta.** `v * count // 100` tiene que mandar el mayor valor posible a la última cubeta, no más allá. Con valores hasta 99 da como mucho 4; admitir el 100 necesitaría una sexta cubeta.
- **El número de cubetas.** Con muy pocas, cada cubeta es grande; con demasiadas, la mayoría están vacías pero hay que visitarlas igual. Con datos bien repartidos, una elección habitual es tener más o menos tantas cubetas como valores.

## Cuándo usarlo

Úsala cuando sepas que los valores están bastante bien repartidos en un rango conocido: fracciones aleatorias entre 0 y 1, o medidas dentro de unos límites conocidos. La misma idea de repartir por tramos está detrás de los histogramas y de dividir una ordenación enorme entre varias máquinas, cada una con un tramo de claves.

## Temas relacionados

- [Ordenación por conteo](learn:sorting/counting-sort): una cubeta por valor, así que ninguna cubeta necesita ordenarse.
- [Ordenación por residuos](learn:sorting/radix-sort): cubetas por cifra, en varias pasadas.
- [Ordenación por inserción](learn:sorting/insertion-sort): la ordenación clásica para las cubetas pequeñas.
