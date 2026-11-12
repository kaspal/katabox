Llama "desordenada" a una pareja de niños cuando el más alto está a la izquierda del más bajo. ¿Cómo cambia un cambio entre vecinos el número de parejas desordenadas?
---
Un cambio entre vecinos cambia el orden de exactamente esa pareja, así que quita como mucho una pareja desordenada, y cambiar una pareja vecina desordenada (como hace el ordenamiento de burbuja) quita exactamente una. La respuesta es el número de parejas desordenadas. Contarlas pareja a pareja cuesta O(n²).
---
Cuenta durante un **ordenamiento por mezcla**: al mezclar dos mitades ordenadas, cuando un elemento de la mitad derecha se coloca antes de los `k` elementos que quedan de la mitad izquierda, estaba desordenado con exactamente esos `k` (todos más altos). Suma `k` cada vez.
