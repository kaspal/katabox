## El error

El bucle borra elementos de la misma lista que está recorriendo:

```python
for x in items:
    if x in bad:
        items.remove(x)
```

Un bucle `for` sobre una lista la recorre por posiciones. Al eliminar un elemento, todos los siguientes se desplazan una posición a la izquierda y el bucle pasa a la posición siguiente, de modo que el elemento que acaba de ocupar la posición actual nunca se revisa. Dos valores vetados seguidos son la víctima típica. Con `items = [7, 7]` y `banned = [7]`, el bucle quita el primer 7 (la lista queda `[7]`), avanza a la posición 1, no encuentra nada y devuelve `[7]` en lugar de `[]`. (Además, `remove` borra la primera aparición del valor y no la actual, lo que disimula aún más el problema.)

## La corrección

No toques `items` y reúne los valores permitidos en una lista nueva:

```python
def without_banned(items: list[int], banned: list[int]) -> list[int]:
    bad = set(banned)
    return [x for x in items if x not in bad]
```

El código inicial de TypeScript tiene el mismo fallo con `splice` dentro de un bucle por índices; devuelve `items.filter((x) => !bad.has(x))` en su lugar.

**Complejidad:** O(n + m) en tiempo y O(n + m) de espacio extra, con n elementos y m valores vetados. (Borrar en el sitio con `remove` o `splice` costaría además O(n) por cada borrado.)
