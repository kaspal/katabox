## El error

La ventana deslizante está bien construida, pero la respuesta se anota demasiado pronto:

```python
        count[c] = count.get(c, 0) + 1
        best = max(best, end - start + 1)
        while len(count) > k:
```

Justo después de añadir `text[end]`, la ventana puede tener `k + 1` letras distintas. El código la mide en ese momento, antes de que el bucle `while` la encoja hasta una ventana válida, así que una ventana inválida puede acabar siendo la respuesta.

Con `text = "aab", k = 1`: tras la segunda `a` la ventana es "aa" (longitud 2). Al añadir `b` pasa a "aab", que tiene dos letras, pero `best` vale 3 antes de que se quiten las `a`. El código devuelve 3; la respuesta es 2. En el ejemplo 2 devuelve 5 (por "bhhhg", que tiene tres letras) en lugar de 4.

## La corrección

Anota la longitud solo cuando la ventana vuelve a ser válida:

```python
    for end, c in enumerate(text):
        count[c] = count.get(c, 0) + 1
        while len(count) > k:
            s = text[start]
            count[s] -= 1
            if count[s] == 0:
                del count[s]
            start += 1
        best = max(best, end - start + 1)
```

En TypeScript, mueve `best = Math.max(best, end - start + 1);` debajo del bucle `while` de la misma forma.

**Complejidad:** O(n) en tiempo, porque cada letra entra y sale de la ventana como mucho una vez; O(1) de espacio extra (como mucho 27 letras en el mapa).
