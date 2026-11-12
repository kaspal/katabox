Un sendero del jardín es una franja de 1 baldosa de ancho y `n` baldosas de largo. El jardinero tiene baldosas cuadradas (1×1), losas cortas (1×2) y losas largas (1×3), tantas de cada como haga falta, y las coloca una tras otra a lo largo del sendero sin huecos ni solapes.

¿De cuántas formas distintas se puede pavimentar el sendero? Dos pavimentos son distintos si las secuencias de piezas de un extremo al otro son distintas. El número crece deprisa: devuélvelo módulo 1 000 000 007.

### Ejemplo 1

```
n = 3
Salida: 4
```

Cuadrada-cuadrada-cuadrada, cuadrada-corta, corta-cuadrada o una losa larga.

### Ejemplo 2

```
n = 1
Salida: 1
```

### Ejemplo 3

```
n = 5
Salida: 13
```

### Restricciones

- 1 ≤ n ≤ 10⁶
