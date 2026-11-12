## Enfoque: una rotación canónica por grabado

Dos grabados son la misma pulsera cuando uno es una rotación del otro. Ser rotación es una equivalencia: cada pulsera es un grupo de grabados. Para contar grupos sin comparar parejas, da a cada grupo un representante **canónico** que cada miembro pueda calcular por su cuenta: la rotación menor en orden alfabético.

```
vistos = conjunto vacío
para code en codes:
    menor = min(code[i:] + code[:i] para i en 0 .. len(code) - 1)
    añadir menor a vistos
devolver tamaño de vistos
```

Los grabados de longitudes distintas tienen rotaciones de longitudes distintas, así que nunca coinciden.

### Paso a paso

| grabado | rotaciones | menor |
| --- | --- | --- |
| lamp | lamp, ampl, mpla, plam | ampl |
| mpla | mpla, plam, lamp, ampl | ampl |
| palm | palm, almp, lmpa, mpal | almp |
| ampl | … | ampl |
| lmpa | lmpa, mpal, palm, almp | almp |

El conjunto es {ampl, almp}: 2 pulseras.

**Complejidad:** O(n · L²) de tiempo, con L ≤ 12, y O(n · L) de espacio. El algoritmo de Booth encuentra la menor rotación en O(L), lo que solo importa con grabados largos. Comparar cada pareja de grabados cuesta O(n² · L²).
