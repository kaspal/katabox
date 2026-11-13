Lee todo el registro de una vez y divídelo en tokens en lugar de leer una línea por llamada: con decenas de miles de líneas, el coste de cada llamada de lectura se acumula.
---
Los tres números se pueden mantener al día mientras recorres las lecturas una sola vez: una suma acumulada, un contador de subidas y la longitud de la racha actual de lecturas iguales. No vuelvas nunca al principio del registro.
---
Guarda `prev`, la lectura anterior. Para cada lectura nueva `x`: súmala; si `x > prev`, cuenta una subida; si `x == prev`, la racha actual crece en 1 y si no, vuelve a empezar en 1; actualiza la racha más larga. Empieza con `racha = mayor = 1` en la primera lectura.
