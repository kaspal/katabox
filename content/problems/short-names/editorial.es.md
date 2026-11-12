## Enfoque: un trie con contadores de paso

Un comienzo `p` de un nombre es una etiqueta válida exactamente cuando el único nombre que empieza por `p` es el propio nombre. Así que hace falta, para muchos prefijos, el número de nombres que empiezan por ellos. Un **trie** lo responde: inserta cada nombre letra a letra y suma 1 a un contador `pasos` en cada nodo del camino. El `pasos` de un nodo es el número de nombres que empiezan por su prefijo.

Después, para cada nombre, baja desde la raíz por sus letras. Las cuentas solo bajan a medida que el prefijo crece, y el primer nodo con `pasos == 1` da la etiqueta más corta. Si todos los nodos del camino tienen `pasos ≥ 2`, el nombre entero es el comienzo de algún otro, y la etiqueta es el nombre entero.

```
para nombre en names: insertar(nombre)     # pasos += 1 por el camino
para nombre en names:
    nodo = raíz
    para k, c en enumerar(nombre):
        nodo = hijo(nodo, c)
        si nodo.pasos == 1: etiqueta = nombre[: k + 1]; cortar
    si no se cortó: etiqueta = nombre
```

### Paso a paso

Con "dog", "door", "cat", "cattle": los nodos "d" y "do" tienen 2 pasos y "dog" tiene 1, así que "dog" necesita sus tres letras; "doo" tiene 1 paso. En "cat" todos los nodos tienen 2 pasos (compartidos con "cattle"), así que la etiqueta es "cat"; para "cattle", "catt" tiene 1 paso.

**Complejidad:** O(total de letras) de tiempo y O(total de letras) nodos. Comparar cada nombre con todos los demás cuesta O(n² · L).
