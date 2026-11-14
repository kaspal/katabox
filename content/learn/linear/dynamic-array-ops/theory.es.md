## La idea

Un arreglo dinámico guarda sus valores uno junto a otro en un solo bloque de memoria, apretados desde la posición 0. `size` dice cuántas casillas están en uso; el resto quedan libres. Para **insertar** en la posición `i`, cada valor desde `i` hasta el final se mueve un lugar a la derecha y abre un hueco. Para **borrar** la posición `i`, cada valor posterior se mueve un lugar a la izquierda y cierra el hueco. Añadir al final es solo insertar en la posición `size`: detrás no hay nada, así que nada se mueve.

Cuando todas las casillas están en uso, la inserción primero **duplica** la memoria: un bloque nuevo el doble de grande, con los valores antiguos copiados. [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array) explica por qué esa copia sale, de media, a `O(1)` por inserción al final.

### Por qué desplazar desde el final

Para abrir un hueco, empieza por el **último** valor: pasa a la casilla libre, el anterior ocupa el sitio que dejó, y así hasta llegar a `i`. Empezar en `i` pisaría `a[i + 1]` antes de que se hubiera movido. Cerrar un hueco es lo contrario: desplaza a la izquierda empezando justo después de la casilla borrada.

## Pasos

1. Insertar `value` en `i`: si `size` es igual a la capacidad, duplica la memoria.
2. Para `k` desde `size` bajando hasta `i + 1`, copia `data[k - 1]` en `data[k]`.
3. Escribe `value` en `data[i]` y suma 1 a `size`.
4. Borrar en `i`: para `k` desde `i` hasta `size - 2`, copia `data[k + 1]` en `data[k]`.
5. Resta 1 a `size`; la última casilla vuelve a estar libre.

## Coste

| Operación | Coste |
|---|---|
| Leer o escribir `a[i]` | `O(1)` |
| Añadir al final | `O(1)` amortizado; `O(n)` en el paso que duplica |
| Insertar en `i` | `O(n − i)`: un desplazamiento por cada valor posterior |
| Borrar en `i` | `O(n − i)` |
| Espacio extra | Hasta `n` casillas libres justo después de duplicar |

## Errores comunes

- **Desplazar en la dirección equivocada.** Abrir el hueco desde la izquierda copia `a[i]` sobre `a[i + 1]`, luego esa copia sobre `a[i + 2]`, y toda la cola acaba siendo un mismo valor repetido.
- **Olvidar `size`.** Las casillas libres siguen guardando valores viejos. Lee solo `data[0..size - 1]` y actualiza `size` en cada inserción y borrado.
- **Bucles que trabajan al principio.** Insertar o borrar en la posición 0 una y otra vez deja un bucle en `O(n²)`. Si necesitas un principio rápido, usa una bicola o un búfer circular.

## Cuándo usarlo

El arreglo dinámico es la lista por defecto en casi todos los lenguajes: compacto, amigo de la caché y con acceso instantáneo por índice. Encaja mejor cuando la mayoría de los cambios ocurren al final. Si dominan las inserciones y borrados en el medio, una lista enlazada o un árbol equilibrado pueden encajar mejor.

## Temas relacionados

- [Coste amortizado: el arreglo dinámico](learn:foundations/dynamic-array): por qué duplicar mantiene baratas las inserciones al final.
- [Ordenación por inserción](learn:sorting/insertion-sort): el mismo desplazamiento a la derecha para abrir un hueco.
- [Cola y búfer circular](learn:linear/queue-ring-buffer): un arreglo fijo que nunca desplaza nada.
- [Lista simplemente enlazada](learn:linear/singly-linked-list): inserciones sin desplazar, pero sin `a[i]` instantáneo.
