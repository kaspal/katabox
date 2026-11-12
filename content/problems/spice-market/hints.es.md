Como se puede llevar cualquier parte de un saco, cada gramo de la bolsa se llena por separado. ¿Cuál es el mejor gramo de especia para meter?
---
Compara las especias por su valor por gramo, `prices[i] / weights[i]`. Un gramo de la especia que más vale por gramo es siempre al menos tan bueno como un gramo de cualquier otra.
---
Ordena los puestos por valor por gramo, de mayor a menor, y llévate sacos enteros mientras quepan. Llena lo que quede con parte del siguiente saco y para. Para comparar sin redondeos, compara `p1 · w2` con `p2 · w1`.
