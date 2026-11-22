## El error

El código inicial es un union-find, y la comprobación `find(a) != find(b)` es correcta. La unión no lo es:

```python
if find(a) != find(b):
    parent[a] = b
    clubs -= 1
```

`parent[a] = b` cambia el enlace de la persona `a`, no el de la raíz de su club. Si `a` ya apuntaba a alguien, ese enlace se pierde: todos los que estaban conectados a través de `a` se quedan fuera del club unido, y las amistades posteriores entre ellos se cuentan como uniones nuevas. El contador puede llegar incluso a 0.

Con `n = 3` y `friendships = [[0, 1], [0, 2], [1, 2]]`:

- `[0, 1]`: las raíces 0 y 1 son distintas, `parent[0] = 1`, 2 clubes.
- `[0, 2]`: las raíces 1 y 2 son distintas, `parent[0] = 2`, 1 club. La persona 1 vuelve a ser raíz, separada de 0.
- `[1, 2]`: las raíces 1 y 2 son distintas, así que el código vuelve a unir: 0 clubes.

El código devuelve `[2, 1, 0]`; la respuesta es `[2, 1, 1]`.

## La corrección

Enlaza las dos raíces, para que se unan los dos clubes completos, y acorta el camino en `find` para que los árboles sigan siendo poco profundos en cadenas largas:

```python
def find(x: int) -> int:
    while parent[x] != x:
        parent[x] = parent[parent[x]]
        x = parent[x]
    return x

for a, b in friendships:
    ra, rb = find(a), find(b)
    if ra != rb:
        parent[ra] = rb
        clubs -= 1
    out.append(clubs)
```

En TypeScript haz los mismos dos cambios: guarda `find(a)` y `find(b)` en `ra` y `rb`, asigna `parent[ra] = rb` cuando sean distintas y añade `parent[x] = parent[parent[x]]` dentro del bucle de `find`.

**Complejidad:** O(n + m log n) en tiempo para m amistades (amortizado, con acortamiento de caminos), y O(n) de espacio extra aparte de la salida.
