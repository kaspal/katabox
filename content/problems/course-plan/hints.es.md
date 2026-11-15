Revisar los requisitos de todos los cursos al empezar cada periodo funciona, pero puede haber tantos periodos como cursos. ¿Qué cursos pueden quedar disponibles en el periodo siguiente? Solo los que dependen de algo aprobado en este periodo.
---
Guarda, para cada curso, cuántas de sus reglas de requisito siguen sin aprobar (una regla repetida cuenta cada vez), y para cada curso la lista de cursos que desbloquea. Cuando se aprueba un curso, baja el contador de cada curso que desbloquea; un curso cuyo contador llega a 0 entra en el periodo siguiente.
---
Algoritmo de Kahn por capas: el primer periodo son todos los cursos con contador 0. Para cada periodo, ordénalo, guárdalo y recorre las listas de desbloqueo de sus cursos reuniendo los cursos cuyo contador baja a 0: ese es el periodo siguiente. Para cuando un periodo quede vacío. Si se aprobaron menos de `n` cursos, algunos están en un ciclo de reglas (o dependen de uno): escribe `IMPOSSIBLE`.
