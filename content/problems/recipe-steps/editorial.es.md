## Enfoque: algoritmo de Kahn con un montículo de mínimos

Trata cada regla `[a, b]` como una flecha `a → b`, y deja que `espera[b]` cuente las flechas que llegan a `b`. Un paso está listo cuando su cuenta es 0.

1. Mete en un montículo de mínimos cada paso con cuenta 0.
2. Saca una y otra vez el paso listo más pequeño, añádelo a la respuesta y baja la cuenta de cada paso al que apunta. Si una cuenta llega a 0, ese paso entra en el montículo.
3. Si salieron menos de `n` pasos, los que quedan se esperan entre sí en un ciclo: devuelve `[]`.

El montículo es lo que hace que el orden sea exactamente el del cocinero: en cada momento hace el paso listo de número más bajo, y ese es el mínimo del montículo. Una regla repetida añade la misma flecha dos veces; se cuenta dos veces y se descuenta dos veces, así que no cambia nada.

```
espera[b] += 1 para cada [a, b]; salidas[a].append(b)
montículo = pasos con espera == 0
mientras montículo:
    s = sacar el menor; orden.append(s)
    para t en salidas[s]:
        espera[t] -= 1
        si espera[t] == 0: meter t
devolver orden si len(orden) == n, si no []
```

### Paso a paso

`n = 4`, reglas 0 → 2, 1 → 2, 3 → 1. Cuentas: el paso 1 espera 1, el paso 2 espera 2. El montículo empieza con {0, 3}: sale el 0 (el 2 pasa a esperar 1), sale el 3 (el 1 queda listo), sale el 1 (el 2 queda listo), sale el 2. Respuesta `[0, 3, 1, 2]`.

**Complejidad:** O((n + m) log n) de tiempo y O(n + m) de espacio. Volver a recorrer todas las reglas y todos los pasos para encontrar el siguiente listo cuesta O(n · (n + m)).
