Recorrer todas las páginas guardadas para encontrar la que hay que quitar cuesta O(capacity) por desalojo. La regla de desalojo ordena las páginas por (recuento de usos, último uso). ¿Cómo mantienes ese orden barato?
---
Agrupa las páginas por recuento de usos. Dentro de un grupo, mantenlas en orden de último uso (la más antigua primero), como la lista de recencia de una caché simple. Una página que se usa pasa del grupo `c` al final del grupo `c + 1`.
---
Guarda un mapa de recuento a un conjunto con orden de inserción de páginas, más `minCount`, el menor recuento con grupo no vacío. Las páginas nuevas ponen `minCount = 1`; cuando una página sale del grupo `minCount` y este queda vacío, `minCount` sube en uno. Para desalojar, quita la página más antigua del grupo `minCount`.
