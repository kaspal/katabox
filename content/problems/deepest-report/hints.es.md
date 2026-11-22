Las respuestas en organigramas pequeños son correctas. Piensa en el organigrama más alto que permiten las restricciones: 5·10⁴ personas en una sola fila, donde la persona `i` reporta a la persona `i + 1` y la última es la jefa.
---
La memoria en `depth` está bien y los niveles que calcula son correctos. El problema es cómo llega `level` hasta la jefa: ¿cuántas llamadas quedan esperando en la pila cuando empieza por la persona 0 en ese organigrama?
---
`level` hace una llamada recursiva por cada responsable de la cadena, así que una cadena de 5·10⁴ personas necesita 5·10⁴ llamadas anidadas y desborda la pila (subir el límite de recursión no es una solución fiable). Sube de forma iterativa: sigue `manager` desde `v` hasta llegar a la jefa o a alguien cuya profundidad ya se conoce, guardando el camino en una lista, y luego recorre el camino de vuelta asignando profundidades.
