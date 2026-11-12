Una carretera está en el bucle exactamente cuando, al quitarla, sus dos pueblos siguen conectados. Comprobarlo carretera a carretera funciona, pero es una búsqueda por carretera: O(n²).
---
Fíjate en un pueblo con una sola carretera (un callejón sin salida). No puede estar en el bucle: un bucle entra y sale de cada uno de sus pueblos por dos carreteras distintas.
---
Quita callejones sin salida una y otra vez: cada vez que quitas uno, su vecino tiene una carretera menos y puede convertirse en un callejón nuevo (usa una cola). Lo que sobrevive es exactamente el bucle; cuéntalo.
