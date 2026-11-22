Prueba una lista con un solo precio, por ejemplo `["4.35"]`. ¿La respuesta es exactamente 435?
---
El problema empieza en el momento en que la cadena del precio se convierte en número. ¿Se puede guardar un valor como 4.35 de forma exacta en coma flotante? ¿Qué le pasa a un error diminuto al multiplicar por 100 y cortar la parte decimal?
---
Los números en coma flotante no pueden representar exactamente la mayoría de los precios decimales, así que `4.35 * 100` da 434.99999999999994 e `int` lo trunca a 434. No uses flotantes: quita el punto y lee cada precio como un número entero de céntimos (`int(p.replace(".", ""))`), y después suma esos enteros.
