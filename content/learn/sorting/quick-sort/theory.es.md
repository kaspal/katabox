## La idea

La ordenación rápida elige un valor, el **pivote**, y **particiona** el tramo: todo valor menor que el pivote va a su izquierda y el resto a su derecha. Tras la partición, el pivote queda en su lugar definitivo, así que los dos lados se pueden ordenar por separado, de la misma manera. Esta versión usa la partición de Lomuto con el último valor como pivote.

## Pasos

1. Toma el último valor del tramo como pivote.
2. Recorre el tramo con `j`; `i` marca dónde termina la parte de los "menores".
3. Cuando `a[j]` es menor que el pivote, intercámbialo a la posición `i` y avanza `i`.
4. Intercambia el pivote a la posición `i`: ya está en su lugar definitivo.
5. Repite en el lado izquierdo y en el derecho. Un tramo de un solo valor ya está ordenado.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n log n)` (cada pivote parte el tramo por la mitad) |
| Caso medio | `O(n log n)` |
| Peor caso | `O(n²)` (cada pivote es el menor o el mayor valor) |
| Espacio extra | `O(log n)` de pila en promedio, `O(n)` en el peor caso |
| Estable | No: un intercambio puede saltar por encima de valores iguales |
| En el sitio | Sí |

## Errores comunes

- **Entrada ordenada o invertida.** Con el último valor como pivote, una entrada ya ordenada (y una invertida) hace que cada partición quede descompensada: un lado vacío y el otro con todo lo demás, y el trabajo crece hasta `n²/2` comparaciones. Prueba la entrada "Invertida" y mira cómo sube el número de comparaciones. Elegir un pivote aleatorio, o la mediana del primero, el del medio y el último, lo soluciona.
- **Muchos valores iguales.** Lomuto manda todos los valores iguales al pivote al mismo lado, así que un arreglo con pocos valores distintos también degenera hacia `n²`. Prueba la entrada "Pocos distintos". Una partición en tres vías lo soluciona (ver "Ordenación rápida: Hoare y tres vías", próximamente).
- **Tramos con un error de uno.** Las llamadas recursivas son `lo .. p - 1` y `p + 1 .. hi`: el pivote nunca vuelve a incluirse.

## Cuándo usarlo

La ordenación rápida es la ordenación de uso general habitual para arreglos en memoria: trabaja en el sitio y su bucle interno es muy rápido. Usa un pivote aleatorio o la mediana de tres, una partición en tres vías si hay muchos duplicados, e inserción para los tramos cortos. Si necesitas estabilidad o un `O(n log n)` garantizado, usa la ordenación por mezcla o por montículo.

## Temas relacionados

- [Ordenación por inserción](learn:sorting/insertion-sort): la opción habitual para los tramos cortos al fondo de la recursión.
- [Ordenación de burbuja](learn:sorting/bubble-sort): una ordenación sencilla `O(n²)` para comparar.
- Ordenación rápida: Hoare y tres vías (próximamente).
