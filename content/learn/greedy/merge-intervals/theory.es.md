## La idea

Una lista de intervalos, como reservas de una sala, puede tener solapes. Fusionarlos significa sustituir cada grupo de intervalos que se solapan por un único intervalo que cubre el mismo tramo, de modo que en el resultado ya no queda ningún solape. Aquí también se fusionan los intervalos que solo se tocan, en los que uno acaba justo donde empieza el siguiente.

El truco es **ordenar por inicio** (si empatan, por fin). Tras ordenar, un intervalo solo puede solaparse con el último fusionado: todos los anteriores acabaron antes de que ese empezara. Así que basta un solo barrido. Guarda una lista de intervalos fusionados. Para cada intervalo en orden, si empieza **no más tarde de donde acaba el último fusionado**, pertenece a él: **amplía** el fin al mayor de los dos fines. Si no, hay un hueco, y el intervalo **abre** uno nuevo.

La elección voraz es dar por cerrado un intervalo fusionado en cuanto aparece un hueco; ningún intervalo posterior puede volver atrás por encima de ese hueco, porque los siguientes empiezan aún más a la derecha. La salida enumera los intervalos fusionados seguidos, inicio y después fin. La entrada se escribe como pares "inicio fin".

### La entrada por defecto

Seis intervalos: `(7, 9)`, `(1, 4)`, `(3, 5)`, `(14, 17)`, `(8, 11)`, `(5, 6)`. Ordenados: `(1, 4)`, `(3, 5)`, `(5, 6)`, `(7, 9)`, `(8, 11)`, `(14, 17)`. `(1, 4)` abre. `(3, 5)` empieza en 3 ≤ 4: se amplía a 5. `(5, 6)` toca en 5: se amplía a 6. `(7, 9)` empieza después de 6: abre. `(8, 11)` empieza en 8 ≤ 9: se amplía a 11. `(14, 17)` abre.

La salida es `[1, 6, 7, 11, 14, 17]`. `(1, 6)` y `(7, 11)` quedan separados: nada cubre el hueco de 6 a 7.

## Pasos

1. Ordena los intervalos por inicio y después por fin.
2. Para cada intervalo `(inicio, fin)`: si la lista no está vacía e `inicio ≤` el fin del último fusionado, pon ese fin a `max(último fin, fin)`.
3. Si no, añade `(inicio, fin)` como un nuevo intervalo fusionado.
4. Devuelve los intervalos fusionados, aplanados.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log n)`: la ordenación; el barrido es `O(n)` |
| Espacio extra | `O(n)` para la copia ordenada y la lista de fusionados |

## Errores comunes

- **Sobrescribir en vez de tomar el máximo.** En `(1, 10)`, `(2, 3)` el segundo intervalo está dentro del primero; poner el fin a 3 perdería casi todo el primero.
- **Saltarse la ordenación.** Sin ordenar, un intervalo puede solaparse con uno fusionado mucho antes, y el barrido nunca mira atrás.
- **La regla de tocarse.** Que `(1, 4)` y `(4, 6)` se fusionen depende del problema. Usa `≤` cuando deban unirse y `<` cuando compartir un extremo no cuente como solape.

## Cuándo usarlo

Úsalo para ordenar calendarios, juntar horas ocupadas o averiguar qué cubre un conjunto de rangos. Los problemas enlazados aplican el mismo barrido. Unir reservas sigue este patrón directamente. En Añadir una franja la lista ya está ordenada, así que la nueva franja se puede colocar en su sitio y hacer el barrido una vez sin ordenar. En Pinceladas, suma las longitudes de los intervalos fusionados para que la pintura aplicada dos veces cuente una sola vez.

## Temas relacionados

- [Selección de actividades](learn:greedy/activity-selection): los mismos intervalos, ordenados por fin, eligiendo el mayor número que no chocan.
- [Sumas prefijas](learn:foundations/prefix-sums): suma 1 en cada inicio y resta 1 tras cada fin, y una suma acumulada cuenta cuántos intervalos cubren cada punto.
- [Dos punteros](learn:searching/two-pointers): recorrer a la vez dos listas ordenadas de intervalos, por ejemplo para encontrar dónde se solapan.
