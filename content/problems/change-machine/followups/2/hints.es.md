Probar todos los grupos de fichas son hasta 2¹⁰⁰ grupos. La tabla por cantidades del
problema base sigue siendo el camino, pero permitía usar una ficha una y otra vez. ¿Qué
tiene que cambiar para que cada ficha se use como mucho una vez?
---
Añade las fichas una a una. Tras procesar algunas, `best[a]` es el menor número de esas
fichas que forman `a`. La ficha nueva o se queda fuera o se usa una vez sobre una tabla
que todavía no la contenía.
---
Para cada ficha `t`, recorre las cantidades **hacia abajo**, desde `amount` hasta `t`, y
mejora `best[a]` con `best[a − t] + 1`. Ir hacia abajo hace que `best[a − t]` todavía no
esté actualizado con `t`, así que `t` cuenta como mucho una vez. Empieza con
`best[0] = 0` y todas las demás cantidades inalcanzables.
