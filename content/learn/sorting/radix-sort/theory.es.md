## La idea

La ordenación por residuos ordena números cifra a cifra, empezando por las unidades: la cifra menos significativa (LSD, por sus siglas en inglés). Cada pasada echa cada valor en una de diez cubetas según su cifra actual y luego recoge las cubetas de la 0 a la 9 de vuelta en el arreglo. El truco es que cada pasada es **estable**: dentro de una cubeta, los valores conservan el orden en que llegaron. Así, tras la pasada de las decenas, dos valores con la misma cifra de las decenas siguen ordenados por sus unidades, gracias a la pasada anterior. Tras la última cifra, todo el arreglo está ordenado.

## Pasos

1. Empieza con `place = 1`, la cifra de las unidades.
2. Crea diez cubetas vacías (`buckets`). Tomando los valores en el orden del arreglo, añade cada `v` a la cubeta `v // place % 10`.
3. Recoge: recorre las cubetas de la 0 a la 9 y escribe sus valores, en orden, desde la posición 0.
4. Multiplica `place` por 10 y repite mientras `place <= max(a)`.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(d · (n + 10))` (`d` = cifras del mayor valor) |
| Caso medio | `O(d · (n + 10))` |
| Peor caso | `O(d · (n + 10))` |
| Espacio extra | `O(n + 10)` para las cubetas |
| Estable | Sí: cada pasada respeta el orden de llegada |
| En el sitio | No |

## Errores comunes

- **Empezar por el extremo equivocado.** LSD va de las unidades hacia arriba. Hacer el mismo bucle desde las centenas hacia abajo desordena lo que fijaron las cifras anteriores.
- **Una pasada inestable.** Si una pasada cambia el orden de los valores dentro de una cubeta, se pierde el trabajo de las pasadas anteriores. Añadir al final de la cubeta y leerla desde el principio la mantiene estable.
- **Números cortos.** El 8 no tiene decenas ni centenas; `v // place % 10` da 0, que es justo lo correcto. En la entrada por defecto, el 8 espera en la cubeta 0 durante las dos últimas pasadas.
- **Valores negativos.** La fórmula de la cifra supone `v ≥ 0`. Ordena los negativos aparte o desplaza antes todos los valores.

## Cuándo usarlo

La ordenación por residuos encaja con claves de ancho fijo: enteros, fechas, códigos postales, cadenas de longitud fija. Su coste crece con el número de cifras, no con `log n`, así que gana a las ordenaciones por comparación cuando hay muchas claves cortas. Las versiones reales suelen usar base 256, un byte por pasada, para necesitar menos pasadas.

## Temas relacionados

- [Ordenación por conteo](learn:sorting/counting-sort): su versión estable con sumas prefijas es la pasada habitual por cifra.
- [Ordenación por cubetas](learn:sorting/bucket-sort): cubetas por tramo de valores en lugar de por cifra.
- [Manipulación de bits: lo básico](learn:foundations/bit-manipulation): las pasadas en base 256 leen cada byte con un desplazamiento y una máscara.
