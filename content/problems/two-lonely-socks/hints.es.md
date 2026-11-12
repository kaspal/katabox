Haz el XOR de todos los números, como con un solo calcetín solitario. Las parejas se anulan. ¿Qué queda?
---
Queda `a XOR b`, donde `a` y `b` son los dos solitarios. No es cero, porque `a ≠ b`. Elige cualquier bit que valga 1: `a` y `b` se diferencian en ese bit.
---
Separa los números según ese bit y haz el XOR de cada grupo por su cuenta. Cada pareja cae en el mismo grupo y se anula, así que un grupo da `a` y el otro da `b`. El bit 1 más bajo de `x` es `x & -x`.
