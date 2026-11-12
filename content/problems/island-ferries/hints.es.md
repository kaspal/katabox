Empieza con `n` grupos de una isla. ¿Qué le hace cada línea de ferri al número de grupos?
---
Una línea entre dos islas del mismo grupo no cambia nada; una línea entre dos grupos distintos los une, así que la cuenta baja en uno. Necesitas una forma rápida de preguntar "¿en qué grupo está esta isla?".
---
Usa conjuntos disjuntos (union-find): `parent[i]` apunta hacia el representante del grupo. `find` sigue los punteros (acortando el camino de paso); `union` enlaza los dos representantes. Cuenta las uniones que sí unen algo y devuelve `n` menos esa cuenta.
