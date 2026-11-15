Un pequeño banco guarda sus cuentas en un libro electrónico: cada cuenta tiene un nombre y un saldo entero. Los empleados escriben órdenes para cambiar saldos y consultarlos. Los cambios se pueden agrupar en transacciones, que se pueden anidar: un empleado puede abrir una transacción dentro de otra y, más tarde, deshacer o conservar los cambios de la más interna.

Este es un **problema de programa**: lee las órdenes de la entrada estándar y escribe las respuestas en la salida estándar.

## Formato de entrada

- La primera línea contiene `Q`, el número de órdenes.
- Después, `Q` líneas, cada una con una de estas órdenes:
  - `SET name amount`: la cuenta `name` pasa a tener `amount`; se crea si no existe.
  - `ADD name amount`: suma `amount` (que puede ser negativo) al saldo de `name`. Si la cuenta no existe, se crea con saldo `amount`.
  - `GET name`: escribe el saldo de `name`, o `NONE` si la cuenta no existe.
  - `COUNT amount`: escribe cuántas cuentas existentes tienen exactamente `amount`.
  - `BEGIN`: abre una transacción nueva dentro de las que ya están abiertas.
  - `ROLLBACK`: deshace todos los cambios hechos desde que empezó la transacción abierta más interna (incluidos los de transacciones dentro de ella que se confirmaron) y la cierra. Una cuenta creada en ese tiempo vuelve a desaparecer.
  - `COMMIT`: cierra la transacción abierta más interna y conserva sus cambios. Pasan a pertenecer a la transacción que la contiene, si la hay, así que un `ROLLBACK` posterior de esa todavía los deshace.

  `ROLLBACK` y `COMMIT` sin ninguna transacción abierta escriben `NO TRANSACTION` y no cambian nada. Las transacciones que siguen abiertas al final simplemente quedan abiertas.

## Formato de salida

Escribe una línea por cada `GET`, cada `COUNT` y cada `ROLLBACK` o `COMMIT` sin transacción abierta, en el orden de las órdenes. Las demás órdenes no escriben nada.

### Ejemplo 1

Entrada:

```
10
SET ana 50
SET bo 50
COUNT 50
BEGIN
ADD ana -20
GET ana
COUNT 50
ROLLBACK
GET ana
COUNT 50
```

Salida:

```
2
30
1
50
2
```

Dentro de la transacción `ana` baja a 30, así que solo `bo` tiene 50. El `ROLLBACK` deja `ana` otra vez en 50.

### Ejemplo 2

Entrada:

```
7
BEGIN
SET cy 5
BEGIN
SET cy 9
COMMIT
ROLLBACK
GET cy
```

Salida:

```
NONE
```

El `COMMIT` pasa el cambio a 9 a la transacción exterior, y su `ROLLBACK` deshace los dos cambios: `cy` no existía cuando empezó.

### Ejemplo 3

Entrada:

```
6
COMMIT
ROLLBACK
ADD dee 7
GET dee
COUNT 0
GET zed
```

Salida:

```
NO TRANSACTION
NO TRANSACTION
7
0
NONE
```

`ADD` crea `dee` con 7. `COUNT 0` no cuenta las cuentas que no existen.

### Restricciones

- 1 ≤ Q ≤ 10⁵
- Cada `name` tiene de 1 a 10 letras minúsculas de la `a` a la `z`.
- `SET`: 0 ≤ amount ≤ 10⁹
- `ADD`: -10⁹ ≤ amount ≤ 10⁹
- `COUNT`: -10¹⁴ ≤ amount ≤ 10¹⁴
