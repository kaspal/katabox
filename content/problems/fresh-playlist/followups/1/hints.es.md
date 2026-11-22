Si un tramo cumple la regla, cualquier tramo más corto dentro de él también la cumple. Eso es lo que permitía usar una ventana deslizante en el problema base.
---
Mantén una ventana `[start, end]` y la cuenta de cada canción dentro de ella. Ampliar la ventana con una emisión solo puede romper la regla para la canción recién añadida.
---
Suma `plays[end]` a su cuenta; mientras esa cuenta pase de 2, quita `plays[start]` de las cuentas y avanza `start`. La ventana vuelve a ser válida; quédate con la mayor longitud que veas.
