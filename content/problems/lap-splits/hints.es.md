Sumar minutos y segundos por separado obliga a llevar un minuto cada 60 segundos. ¿Hay una sola unidad que haga fácil sumar y comparar?
---
Convierte cada vuelta a segundos: `m:ss` es `m · 60 + ss`. El total es una suma normal y la vuelta más rápida un mínimo normal. No compares los tiempos como texto: como cadena, `10:00` va antes que `9:59`.
---
Para cada corredor, lee el nombre, `k` y las `k` vueltas siguientes; pásalas a segundos y toma la suma y el mínimo. Vuelve a escribir un tiempo como `total / 60` (división entera) para los minutos, dos puntos, y `total % 60` con dos cifras para los segundos.
