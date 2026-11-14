Probar todas las particiones una a una es imposible: un mensaje de 1 y 2 tiene un número de lecturas que crece como los números de Fibonacci. Cuenta las lecturas sin enumerarlas.
---
Sea `formas[i]` el número de lecturas de los primeros `i` dígitos. La última letra de una de esas lecturas es o bien el dígito `i` solo (vale salvo que sea `0`), o bien los dos dígitos `i-1, i` juntos (valen si forman un número del 10 al 26).
---
`formas[0] = 1`, y `formas[i] = (dígito i ≠ 0 ? formas[i-1] : 0) + (dígitos i-1, i forman 10..26 ? formas[i-2] : 0)`, todo módulo 1 000 003. La respuesta es `formas[n]`; bastan dos variables en lugar de un arreglo entero. No olvides saltarte el `#`.
