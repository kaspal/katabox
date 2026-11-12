## Enfoque: pelar los callejones sin salida

Cuenta las carreteras de cada pueblo (su **grado**). Un pueblo de grado 1 es un callejón sin salida y no puede estar en el bucle, porque un bucle llega a cada uno de sus pueblos por una carretera y sale por otra. Quítalo: su único vecino pierde una carretera. Eso puede convertir al vecino en un callejón nuevo, y así sucesivamente.

```
cola = pueblos de grado 1
mientras cola:
    v = sacar; quitado[v] = cierto
    para cada vecino u de v no quitado:
        grado[u] -= 1
        si grado[u] == 1: meter u
devolver número de pueblos no quitados
```

**Por qué lo que queda es el bucle.** Los pueblos del bucle conservan siempre sus dos carreteras del bucle, así que su grado nunca baja de 2 y nunca se quitan. Todos los demás pueblos cuelgan del bucle en ramas con forma de árbol, y un árbol finito siempre tiene una hoja, así que el pelado sigue hasta que desaparecen todas las ramas. Sobrevive exactamente el bucle.

### Paso a paso

Ejemplo 3: los callejones son 5 y 6. Quitar el 5 convierte el 4 en callejón; quitar el 6 deja el pueblo 0 con grado 2; quitar el 4 deja el pueblo 2 con grado 2. Quedan los pueblos 0, 1, 2 y 3: 4.

**Complejidad:** O(n) de tiempo y memoria: cada pueblo se quita como mucho una vez y cada carretera se mira dos veces. Quitar cada carretera por turnos y buscar otro camino cuesta O(n²).
