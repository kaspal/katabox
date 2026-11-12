Los códigos llegan hasta 10⁹, así que un arreglo indexado por código es demasiado grande. ¿Qué estructura asocia un código a un número de unidades y devuelve 0 para códigos que nunca ha visto?
---
Un mapa hash de artículo a unidades en la estantería. `"stock"` suma, `"count"` lee (0 si falta).
---
En `"sell"`, lo vendido es `min(qty, unidades)`: informa de ello y réstalo de la estantería.
