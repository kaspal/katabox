A una traductora autónoma le ofrecen `n` encargos pequeños. Cada encargo lleva exactamente un día, tiene un plazo (debe hacerse ese día o antes) y paga unos honorarios. Los días se numeran 1, 2, 3, … desde hoy, y ella hace como mucho un encargo por día. Puede rechazar cualquier encargo.

Este es un **problema de programa**: lee las ofertas de la entrada estándar y escribe el mayor total de honorarios que puede ganar en la salida estándar.

## Formato de entrada

- La primera línea contiene `n`, el número de encargos.
- Después, `n` líneas, cada una `deadline fee`: el último día en que se puede hacer el encargo y lo que paga.

## Formato de salida

Escribe una línea: el mayor total de honorarios de un conjunto de encargos que se puedan hacer todos dentro de plazo, uno por día.

### Ejemplo 1

Entrada:

```
5
2 50
1 10
2 40
3 20
1 30
```

Salida:

```
110
```

Haz el encargo de 40 el día 1, el de 50 el día 2 y el de 20 el día 3. Los días 1 y 2 solo admiten dos de los cuatro encargos con plazo hasta el día 2, y los de 50 y 40 son los dos mejores.

### Ejemplo 2

Entrada:

```
1
1000000000 7
```

Salida:

```
7
```

### Ejemplo 3

Entrada:

```
4
1 5
1 9
1 2
1 9
```

Salida:

```
9
```

Todos los encargos vencen el día 1, así que solo se puede hacer uno.

### Restricciones

- 1 ≤ n ≤ 10⁵
- 1 ≤ deadline ≤ 10⁹
- 1 ≤ fee ≤ 10⁹
