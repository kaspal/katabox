La regla voraz "toma la charla que termina antes" ya no funciona: una charla corta y poco
interesante puede bloquear otra larga y valiosa. Tienes que comparar opciones, no
quedarte con una.
---
Ordena las charlas por hora de fin. Para cada charla hay dos opciones: saltarla, o tomarla
junto con el mejor programa entre las charlas que terminan antes de su inicio (o justo en
él).
---
Sea `best[i]` el mejor total usando las primeras `i` charlas por hora de fin. Para la
charla `i`, busca en binario en los finales cuántas charlas terminan en su inicio o antes
(`j`); entonces `best[i + 1] = max(best[i], best[j] + score)`.
