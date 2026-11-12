La respuesta es uno más que el mayor número de pasos desde alguien hasta la fundadora. Subir desde cada persona por separado es correcto, pero en una línea larga de descendientes cuesta unos n²/2 pasos.
---
Dale la vuelta al arreglo de padres: para cada persona, haz la lista de sus hijos. Así el árbol se puede recorrer **hacia abajo** desde la fundadora.
---
Avanza generación a generación (en anchura): empieza por la fundadora, luego todos sus hijos, luego todos los hijos de estos, … Cuenta cuántas rondas hay hasta que no queda nadie.
