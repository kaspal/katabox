## Enfoque: primero la que termina antes

Ordena las charlas por hora de fin. Lleva `libre`, el momento en que el visitante vuelve a estar libre (empieza en −∞). Para cada charla en orden, si empieza en `libre` o después, asiste y pon `libre` a su hora de fin.

**Por qué es óptimo:** toma cualquier horario óptimo. Su primera charla no termina antes que la charla que termina primero de todas, así que cambiarla por la que termina antes mantiene el horario válido e igual de largo. Repetir el argumento con el tiempo restante da el horario avaricioso.

### Paso a paso

Ejemplo 1 ordenado por fin: 8–9, 9–11, 10–12, 11–13, 12–15. Asiste a 8–9 (libre a las 9), 9–11 (libre a las 11), salta 10–12, asiste a 11–13 (libre a las 13), salta 12–15: 3 charlas.

**Complejidad:** O(n log n) por la ordenación. Un DP que, para cada charla, recorre todas las anteriores buscando la mejor compatible es O(n²).
