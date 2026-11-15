Ordena las ofertas por hora de fin. Después piensa en la última oferta de ese orden: o el mejor plan la deja fuera, o la toma. Si la toma, ¿qué puede contener el resto del plan?
---
Sea `best[i]` el mejor plan que usa solo las `i` primeras ofertas por hora de fin. Tomar la oferta `i` solo deja sitio para ofertas que terminan, como tarde, en su `start`, y en el orden ordenado forman un prefijo que puedes encontrar con una búsqueda binaria. Mete el desempate en el propio valor: compara los planes como el par `(total, −count)`, donde mayor es mejor.
---
Ordena por `end` y guarda los `ends` ordenados. `best[0] = (0, 0)`. Para la oferta `i` (desde 0) con `start` y `pay`, sea `j` el número de ofertas entre las `i` primeras con `end ≤ start` (`bisect_right(ends, start, 0, i)`). Entonces `best[i + 1] = max(best[i], (best[j].total + pay, best[j].minus_count − 1))`. La respuesta es `best[n].total` y `−best[n].minus_count`.
