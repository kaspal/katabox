Ordenar todo y quedarte con las `k` primeras funciona. ¿Puedes evitar guardar más de `k` puntuaciones a la vez?
---
Guarda las `k` mejores vistas hasta ahora. Cuando llega una nueva, solo importa si supera a la peor de esas.
---
Un montículo de mínimos de tamaño `k` te da la peor guardada en O(1) y la sustituye en O(log k). Al final, ordena las `k` guardadas de mayor a menor.
