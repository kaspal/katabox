Para un límite fijo `L`, puedes comprobar si alguna ruta nunca da un paso más duro que `L`: recorre la cuadrícula usando solo pasos de como mucho `L`. La respuesta es el menor `L` con el que se llega al final.
---
Probar todos los límites de uno en uno es lento, pero la comprobación es monótona: si `L` sirve, cualquier límite mayor sirve. Eso permite una búsqueda binaria sobre `L`. También hay un camino directo.
---
Usa el algoritmo de Dijkstra donde la "longitud" de una ruta es su paso más duro: desde un punto alcanzado con paso más duro `d`, a un vecino se llega con `max(d, |diferencia|)`. Un montículo de mínimos expande siempre el punto con el menor `d`; la primera vez que sale el punto de abajo a la derecha, su `d` es la respuesta.
