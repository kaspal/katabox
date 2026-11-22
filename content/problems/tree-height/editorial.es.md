## El error

El caso base está mal:

```python
    if root is None:
        return 1
```

Un hijo que falta no es un nivel, pero el código inicial lo cuenta como uno. Cada hoja informa entonces altura 2 en lugar de 1, y ese 1 de más se arrastra hasta la raíz: todo árbol no vacío sale con un nivel de más y el árbol vacío da 1 en lugar de 0.

Con `root = [7]`, el código calcula `1 + max(1, 1) = 2`, pero un solo nodo tiene altura 1.

## La corrección

```python
    if root is None:
        return 0
```

El código TypeScript tiene el mismo caso base (`if (root === null) return 1;`); cámbialo para que devuelva 0.

**Complejidad:** O(n) en tiempo y O(h) de espacio extra por la recursión, donde h ≤ 1000 es la altura.
