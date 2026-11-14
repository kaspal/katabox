## La idea

Para encontrar el k-ésimo menor valor no hace falta ordenarlo todo. La selección rápida usa la misma partición de Lomuto que la ordenación rápida: tras una partición, el pivote queda en su posición definitiva `p`. Aquí `k` cuenta desde 1 pero las posiciones cuentan desde 0, así que la respuesta es el valor que caería en la posición `k − 1`. Si `p` es justo `k − 1`, el pivote es la respuesta. Si no, la respuesta está en un solo lado: a la derecha de `p` cuando `p < k − 1`, a su izquierda cuando `p > k − 1`. La selección rápida descarta el otro lado y vuelve a particionar. Como solo se conserva un lado, el ejemplo usa un bucle que estrecha `lo..hi` en lugar de una llamada recursiva.

## Pasos

1. Pon `lo = 0` y `hi = len(a) - 1`.
2. Particiona `lo..hi` con el último valor como pivote: los valores menores se intercambian hacia delante y luego el pivote va a la posición `p`.
3. Si `p == k - 1`, devuelve `a[p]`.
4. Si `p < k - 1`, quédate con el lado derecho: `lo = p + 1`. Si no, con el izquierdo: `hi = p - 1`.
5. Repite desde el paso 2.

## Coste

| Caso | Coste |
|---|---|
| Mejor caso | `O(n)` (el primer pivote cae en `k − 1`) |
| Caso medio | `O(n)`: si cada pivote parte el tramo por la mitad, `n + n/2 + n/4 + … < 2n` |
| Peor caso | `O(n²)` (cada pivote es el menor o el mayor de lo que queda) |
| Espacio extra | `O(1)`: un bucle, sin recursión |
| Estable | No |
| En el sitio | Sí: reordena el arreglo |

## Errores comunes

- **Errores de uno con k.** k empieza en 1 (el 1.º menor es el mínimo), pero las posiciones empiezan en 0, así que el objetivo es `k − 1`. Comparar `p` con `k` devuelve el valor siguiente.
- **Reordena la entrada.** El arreglo vuelve medio ordenado. Trabaja sobre una copia si el orden original importa.
- **Pivotes malos.** Con el último valor como pivote, una entrada ordenada o invertida hace que cada partición quite un solo valor, y el trabajo crece hacia `n²`. Un pivote aleatorio lo hace muy improbable.

## Cuándo usarlo

Úsala para una mediana, un percentil o la k-ésima mejor puntuación. Para una sola consulta gana a ordenar: `O(n)` en promedio frente a `O(n log n)`. Para muchas consultas sobre los mismos datos, ordena una vez y lee posiciones. Muchas bibliotecas la incluyen, como `nth_element` de C++ y `partition` de NumPy.

## Temas relacionados

- [Ordenación rápida](learn:sorting/quick-sort): la misma partición, recurriendo en los dos lados.
- [Ordenación rápida: Hoare y en tres partes](learn:sorting/quick-sort-3way): otras particiones que también sirven aquí.
- Búsqueda binaria (próximamente): también descarta un lado en cada paso.
