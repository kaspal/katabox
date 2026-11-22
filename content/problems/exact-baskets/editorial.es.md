## El error

El código inicial es una búsqueda con vuelta atrás sobre los pesos ordenados, pero su bucle nunca vuelve atrás:

```python
for j in range(i, len(items)):
    pick.append(items[j])
    go(j + 1, total + items[j])
```

Cada tarro se añade a `pick` y se queda ahí cuando vuelve la llamada recursiva. El siguiente tarro del bucle se añade entonces encima del anterior en lugar de sustituirlo. El total que se pasa sigue siendo correcto, pero `pick` no para de crecer: la única cesta que llega a comprobarse con exactamente `k` tarros es la primera (los `k` tarros más ligeros), y a partir de ahí `len(pick)` siempre supera `k`, así que nada más puede coincidir.

Con `weights = [1, 2]`, `k = 1` y `target = 2`: el bucle primero añade 1 y comprueba la cesta `[1]` (total 1, no coincide). Después añade 2 sin quitar el 1, así que `pick = [1, 2]` con longitud 2, que no es `k`. El código devuelve `[]`; la respuesta es `[[2]]`.

## La corrección

Deshaz cada elección después de explorarla, para que `pick` vuelva a su estado anterior antes de probar el siguiente tarro:

```python
for j in range(i, len(items)):
    pick.append(items[j])
    go(j + 1, total + items[j])
    pick.pop()
```

En TypeScript, añade `pick.pop();` después de la llamada recursiva `go(j + 1, total + items[j])` en el mismo bucle.

Como los pesos están ordenados y el bucle los prueba de menor a mayor, cada cesta sale en orden ascendente y las cestas salen en orden lexicográfico sin ordenar nada más.

**Complejidad:** O(2ⁿ · n) en tiempo en el peor caso (como mucho 2ⁿ elecciones parciales, y copiar una cesta cuesta O(k)), y O(n) de espacio extra para la recursión y `pick`, aparte de la salida.
