## La idea

Una mochila admite como mucho cierta **capacidad** de peso. Cada objeto tiene un peso y un valor, y esta vez se puede meter **una parte** de cualquier objeto: medio objeto pesa la mitad y vale la mitad. Piensa en sacos de arroz o de especias que se venden al peso. ¿Cuál es el mayor valor total?

Cada objeto tiene un **valor por unidad de peso**, su razón `valor / peso`. La regla voraz es meter primero el objeto con la mejor razón, entero si cabe, después el siguiente mejor, y así sucesivamente. El primer objeto que no cabe se corta para llenar justo el hueco, y la mochila queda llena. Si dos razones empatan, se respeta el orden de la entrada.

¿Por qué es seguro? Supón una carga que lleva algo de peso de un objeto peor mientras otro mejor no está metido entero. Cambia una unidad de peso: sale una unidad del objeto peor y entra una del mejor. El peso sigue igual y el valor no baja. Repitiendo el cambio, cualquier carga se convierte en la voraz sin perder valor.

Para ser exacto, el ejemplo devuelve el valor total como un número mixto: `[entero, numerador, denominador]`, la parte entera y después la fracción que sobra, irreducible. Cuando caben todos los objetos, la fracción es `0/1`. La entrada se escribe como la capacidad seguida de pares "peso valor".

### La entrada por defecto

Capacidad 10; objetos `(4, 12)`, `(6, 9)`, `(3, 12)`, `(5, 5)` como peso y valor. Las razones son 3, 1,5, 4 y 1, así que el orden es objetos 2, 0, 1, 3. El objeto 2 cabe: valor 12, hueco 7. El objeto 0 cabe: valor 24, hueco 3. El objeto 1 pesa 6 pero solo quedan 3, así que entra la mitad: `9 · 3 / 6 = 27 / 6 = 4 1/2`. El total es `24 + 4 1/2 = 28 1/2`.

La salida es `[28, 1, 2]`. El objeto 3 no llega a entrar en juego.

## Pasos

1. Ordena los objetos por razón, de mayor a menor; los empates respetan el orden de la entrada.
2. Empieza con `hueco = capacidad` y un valor de 0.
3. Para cada objeto: si su peso cabe en `hueco`, suma su valor y resta su peso.
4. Si no, suma `valor · hueco / peso` y para: la mochila está llena.
5. Devuelve el total como `[entero, numerador, denominador]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n log n)`: la ordenación; el recorrido que llena la mochila es `O(n)` |
| Espacio extra | `O(n)` para el orden |

## Errores comunes

- **Usarlo para la mochila 0/1.** Con capacidad 10 y objetos `(6, 12)`, `(5, 9)`, `(5, 9)`, la mejor razón es la del primer objeto, pero una vez dentro ya no cabe ningún otro entero: 12, mientras que los otros dos dan 18. El algoritmo voraz acierta aquí solo porque los objetos se pueden partir.
- **Totales en coma flotante.** Una parte como `1/3` no tiene un valor exacto en coma flotante, y las sumas de esas partes pueden desviarse. Guarda un numerador y un denominador, como hace el ejemplo, o compara razones multiplicando en cruz.
- **Olvidar parar.** Tras el objeto cortado la mochila está llena; no hay que sumar los objetos siguientes.

## Cuándo usarlo

Úsalo siempre que la mercancía se pueda dividir: comprar especias o combustible al peso, o repartir un presupuesto entre tareas que se pagan por horas. El problema enlazado, Mercado de especias, sigue este patrón.

## Temas relacionados

- [Mochila 0/1](learn:dynamic-programming/knapsack-01): objetos que no se pueden partir, resueltos con una tabla.
- [Corte de barras (mochila sin límite)](learn:dynamic-programming/rod-cutting): piezas enteras que se pueden repetir.
- [Selección de actividades](learn:greedy/activity-selection): qué hace que un problema se preste a un algoritmo voraz.
