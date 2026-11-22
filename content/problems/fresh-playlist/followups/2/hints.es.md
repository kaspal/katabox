Para una petición, un corte voraz funciona: haz cada bloque tan largo como se pueda,
empezando donde acabó el anterior. ¿Por qué nunca es peor? Y con 10⁵ peticiones, el propio
recorrido voraz (hasta 10⁵ bloques por petición) sigue siendo demasiado lento.
---
Desde la emisión `i`, el bloque sin repeticiones más largo acaba justo antes de `next[i]`,
y `next[i]` nunca baja al crecer `i`; una pasada de la ventana del problema base lo calcula
para todo `i`. Una petición es entonces "empieza en `a`, sigue `next` mientras no pase de
`b` y cuenta los pasos".
---
Seguir `next` muchas veces es justo para lo que sirven los saltos binarios: `jump[k][i]` es
adónde llevan `2^k` bloques que empiezan en `i` (`jump[0] = next`,
`jump[k][i] = jump[k−1][jump[k−1][i]]`). En una petición, da los saltos más largos que no
pasen de `b`, sumando `2^k` bloques cada vez; la respuesta es uno más que los bloques
saltados.
