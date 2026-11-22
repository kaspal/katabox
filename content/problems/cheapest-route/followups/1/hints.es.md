Dijkstra normal encuentra la ruta más barata pero no mira cuántas carreteras usa, y una
ruta barata con demasiadas carreteras no sirve. ¿Qué información extra necesita cada
pueblo?
---
Piensa por rondas: si conoces el peaje más barato hasta cada pueblo usando como mucho `r`
carreteras, ¿cómo obtienes el más barato usando como mucho `r + 1`?
---
Haz `k` rondas de relajación sobre todas las carreteras, en los dos sentidos. En cada
ronda, lee los costes de la ronda anterior y escribe en una copia, para que una ronda
nunca encadene dos carreteras. Para antes si una ronda no cambia nada.
