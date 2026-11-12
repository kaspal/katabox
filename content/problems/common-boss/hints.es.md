Para una disputa puedes subir desde `a` marcando cada jefe y luego subir desde `b` hasta dar con alguien marcado. Con 5·10⁴ disputas en una jerarquía de 5·10⁴ niveles, es demasiado lento. Hace falta subir muchos niveles de golpe.
---
Calcula primero la profundidad de cada socio (niveles por debajo de la presidenta). Sube al más profundo hasta la profundidad del otro; después, si son distintos, sube a los dos **a la vez** hasta que sus jefes coincidan. Subir `d` niveles se hace en O(log d) saltos si conoces el jefe de cada socio 1, 2, 4, 8, … niveles por encima.
---
**Salto binario**: `up[0][v] = boss[v]` (la presidenta se apunta a sí misma) y `up[k][v] = up[k − 1][up[k − 1][v]]`. Para igualar profundidades, salta según las potencias de dos de la diferencia. Después, para `k` de mayor a menor: si `up[k][a] ≠ up[k][b]`, salta con los dos. La respuesta es `up[0][a]` (o `a` si ya eran iguales).
