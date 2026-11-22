Recorrer todos los títulos en cada consulta, o reconstruirlo todo después de cada cambio, es demasiado lento para 3·10⁴ operaciones. ¿Qué estructura respondía a los prefijos en el problema base, y puede cambiar?
---
En un trie, guarda en cada nodo cuántos títulos pasan por él. Una consulta es entonces solo bajar por las letras escritas.
---
Añadir un título baja por sus letras, creando los nodos que falten, y suma 1 al contador de cada nodo del camino; retirarlo recorre el mismo camino y resta 1. Una consulta lee el contador del nodo donde acaba el texto escrito, o 0 si el camino se corta.
