Deshacer siempre afecta a la fusión vigente más reciente. ¿Qué estructura de datos te da "el elemento más reciente"?
---
La compresión de caminos recoloca a muchos jugadores a la vez, y eso es difícil de deshacer. La unión por tamaño por sí sola ya mantiene cada árbol con altura O(log n), y así cada fusión cambia solo un enlace de padre y un tamaño.
---
En cada fusión, apila la raíz que se colgó, o una marca de "no cambió nada". Con `[0]`, desapila: si es una raíz real, haz que vuelva a ser su propio padre, resta su tamaño a la raíz a la que se había unido y suma un club.
