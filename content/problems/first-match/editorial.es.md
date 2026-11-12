## Enfoque: un trie que recuerda su palabra más pequeña

En un trie, las palabras que empiezan por un prefijo son justo aquellas cuyo camino pasa por el nodo del prefijo. Así que la respuesta para un prefijo solo depende de ese nodo: guarda en cada nodo la **palabra más pequeña** que pasa por él.

```
para palabra en sorted(words):       # en orden alfabético
    nodo = raíz
    para c en palabra:
        nodo = hijo(nodo, c)         # se crea si falta
        si nodo.mejor es None: nodo.mejor = palabra   # la primera en llegar es la menor
respuesta(prefijo):
    baja por el prefijo; devuelve nodo.mejor, o "" si falta una letra
```

Insertar en orden alfabético hace que la primera palabra que llega a un nodo sea la menor de todas las que pasan por él. (Sin ordenar, compara cada palabra con la guardada.) Una palabra más corta va antes que las palabras más largas que empiezan por ella, como pide el enunciado.

### Paso a paso

Ordenadas: apple, peach, pear, plum. "peach" llega primero a los nodos p, pe, pea y peac, así que "pe", "p" y "peac" responden "peach"; "a" responde "apple"; "b" no tiene nodo, así que "".

**Complejidad:** O(n log n · L) para ordenar, O(total de letras) para construir el trie y O(L) por consulta. Recorrer todas las palabras para cada prefijo cuesta O(n · q · L).
