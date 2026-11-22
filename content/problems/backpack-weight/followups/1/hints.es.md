Podrías tratar cada copia como un objeto propio y usar la tabla del problema base: son
hasta 5000 objetos sobre 5000 capacidades. Funciona, pero ¿puedes arreglártelas con menos
"objetos"?
---
Para permitir cualquier número de copias entre 0 y `c` no necesitas `c` copias sueltas.
Paquetes de 1, 2, 4, 8, ... copias, más un paquete con lo que sobre, pueden sumar
cualquier número entre 0 y `c`.
---
Divide las `counts[i]` copias de cada objeto en paquetes de 1, 2, 4, ... copias y un
último paquete con el resto. Trata cada paquete como un objeto 0/1 con `k` veces el peso
y el valor, y usa la tabla del problema base (capacidades de mayor a menor) con todos los
paquetes.
