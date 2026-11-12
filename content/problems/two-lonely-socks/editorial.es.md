## Enfoque: XOR y después separar por un bit en el que difieren

`x XOR x = 0` y `x XOR 0 = x`, y al XOR no le importa el orden. Así que el XOR de toda la lista anula cada pareja y deja `t = a XOR b`, los dos solitarios combinados.

Como `a ≠ b`, `t` tiene al menos un bit a 1, y en ese bit `a` y `b` difieren. Toma el más bajo, `bit = t & -t`. Ahora haz por separado el XOR de los números que tienen ese bit y el de los que no. Las dos copias de un número emparejado caen en el mismo grupo y se anulan, así que el XOR de un grupo es `a` y el del otro es `b`.

```
t = XOR de todos los números
bit = t & -t
a = XOR de los números con (id & bit) != 0
b = t XOR a
devolver [min(a, b), max(a, b)]
```

### Paso a paso

`ids = [8, 5, 13, 5, 2, 8]`: `t = 13 XOR 2 = 1101₂ XOR 0010₂ = 1111₂`, así que `bit = 1`. Los impares son 5, 13, 5, cuyo XOR es 13; después `b = 15 XOR 13 = 2`. Respuesta `[2, 13]`.

**Complejidad:** O(n) de tiempo y O(1) de espacio adicional. Buscar la pareja de cada número recorriendo la lista cuesta O(n²).
