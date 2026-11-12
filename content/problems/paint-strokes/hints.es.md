Marcar cada centímetro es imposible: la valla mide hasta 10⁹ cm. Piensa en pinceladas enteras.
---
Si las pinceladas están ordenadas por `start`, cada una o se solapa (o se toca) con el tramo pintado que vas formando, o empieza uno nuevo tras un hueco.
---
Ordena por `start`. Guarda el tramo actual `[runStart, runEnd)`. Para cada pincelada: si `start ≤ runEnd`, alarga `runEnd = max(runEnd, end)`; si no, suma `runEnd − runStart` al total y empieza un tramo nuevo. Al final suma el último tramo.
