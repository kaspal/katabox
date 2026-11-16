# Problemas conocidos

Problemas y límites conocidos de KataBox 1.0.0-beta.1. Cada punto explica qué pasa y, cuando
existe, qué se puede hacer.

[English](known-issues.md)

## Instalación y actualización

### Windows SmartScreen muestra una advertencia sobre el instalador
**Qué pasa:** los instaladores todavía no tienen firma de código. La primera vez que ejecutas
uno, Windows puede mostrar "Windows protegió tu PC".
**Solución:** haz clic en **Más información** y luego en **Ejecutar de todas formas**. Si
quieres confirmar que el archivo es el original, compáralo con las sumas SHA-256 de la página
de la versión.

### No hay actualizaciones automáticas
**Qué pasa:** KataBox no busca ni descarga versiones nuevas.
**Solución:** descarga el instalador nuevo desde la página de versiones e instálalo sobre la
versión actual. Tu progreso, tus borradores y tu configuración se conservan.

### Solo para Windows
**Qué pasa:** esta versión no incluye una compilación para Linux. El soporte para Linux está
previsto.
**Solución:** ninguna por ahora.

### El primer uso de un lenguaje requiere una descarga
**Qué pasa:** la app no incluye ningún entorno de ejecución. La primera vez que usas un
lenguaje, KataBox descarga su entorno y su servidor de lenguaje. Tamaños aproximados en
Windows (descarga / en disco):

| Lenguaje | Descarga | En disco |
| --- | --- | --- |
| Python | 98 MB | 297 MB |
| JavaScript/TypeScript | 46 MB | 140 MB |
| PHP | 87 MB | 103 MB |
| Rust | 244 MB | 959 MB |

Python y JavaScript/TypeScript comparten Node.js, así que instalar los dos ocupa menos que la
suma. Los problemas de SQL usan el entorno de Python, así que necesitan Python instalado.
**Solución:** instala los lenguajes mientras tengas conexión; después, KataBox funciona sin
conexión. Si usas un firewall o una lista de hosts permitidos en un proxy, permite
`github.com` y sus hosts de recursos de versiones, `nodejs.org`, `windows.php.net`,
`downloads.php.net`, `static.rust-lang.org` y `registry.npmjs.org`. KataBox usa el proxy del
sistema; no admite scripts PAC/WPAD.

## Editor

### Sin servidor de lenguaje para PHP en Windows
**Qué pasa:** el código PHP tiene resaltado de sintaxis, pero no diagnósticos ni
autocompletado. El servidor de lenguaje de PHP (phpactor) necesita una extensión de PHP que
las compilaciones de PHP para Windows no tienen. La línea de estado del editor lo indica.
**Solución:** ninguna. Ejecutar y Enviar funcionan con normalidad.

### Sin servidor de lenguaje para SQL
**Qué pasa:** SQL no tiene diagnósticos de un servidor. El editor sí completa los nombres de
tablas y columnas del problema y las palabras clave de SQL.
**Solución:** usa Ejecutar para revisar la consulta; un error de sintaxis o un nombre
desconocido se informa como error de compilación.

### Algunos módulos de Node.js aparecen subrayados en problemas de programa en TypeScript
**Qué pasa:** en los problemas de programa, el verificador de tipos del editor conoce solo
una parte pequeña de Node.js: `fs` / `node:fs` (`readFileSync`, `readSync`, `writeSync`),
`process`, `Buffer` y `require`. Si importas otro módulo integrado, como `node:readline`, el
editor puede marcarlo como error aunque el programa se ejecute bien.
**Solución:** lee la entrada con `readFileSync(0)`, como hace el código inicial, o ignora el
aviso; la evaluación no se ve afectada.

### Las pruebas personalizadas no se guardan
**Qué pasa:** las pruebas personalizadas que agregas en un problema se borran al abrir otro
problema o al cerrar la app.
**Solución:** guarda las entradas que quieras reutilizar en las notas del problema, que sí se
guardan.

## Depurador

### No hay depurador para PHP ni Rust
**Qué pasa:** el depurador por repetición y el visualizador funcionan solo en Python y
JavaScript/TypeScript. PHP y Rust no tienen depurador.
**Solución:** depura una solución en PHP o Rust con impresiones, o prueba el caso que falla en
Python o TypeScript.

### Límites del depurador de TypeScript en problemas de programa
**Qué pasa:** un programa en TypeScript se repite como una sola función, así que algunos
programas no se pueden depurar:
- importaciones de cualquier cosa que no sea un módulo integrado de Node, y `await` en el
  nivel superior: el depurador los rechaza e indica el motivo;
- programas que leen la entrada estándar con eventos de `process.stdin`: no se pueden
  repetir.
**Solución:** lee toda la entrada de una vez con `readFileSync(0)` (como el código inicial) y
mantén el código síncrono. Esto afecta solo a la depuración, no a la evaluación.

### Los programas que dependen del tiempo o del azar pueden dejar de repetirse
**Qué pasa:** cada paso del depurador vuelve a ejecutar tu código desde el principio. Si el
código depende del reloj, de `crypto` o de temporizadores, una repetición puede seguir otro
camino y el depurador se detiene con un mensaje de divergencia. `Math.random` (TypeScript) y
`random` (Python) usan una semilla fija, así que no causan problemas.
**Solución:** quita el código que depende del tiempo mientras depuras.

### Las ejecuciones largas se recortan en el depurador de TypeScript
**Qué pasa:** una traza de TypeScript registra como máximo 15 000 pasos (Python: 200 000).
Con una entrada grande, la traza se recorta, y cada paso cerca del final puede tardar unos
segundos.
**Solución:** depura con una entrada pequeña: una prueba pública, una prueba personalizada o
el resultado de "Buscar una entrada más pequeña que falle".

## Evaluación y perfilado

### El perfilador de complejidad da una estimación
**Qué pasa:** el perfilador siempre dice "Estimada". Mide tu código con varios tamaños de
entrada en tu equipo y ajusta una curva, así que una medición con ruido puede dar una clase
equivocada, por ejemplo O(n log n) para una solución O(n). 95 de los 209 problemas tienen
perfil.
**Solución:** tómalo como una pista, no como un veredicto. Compara tu línea con la de la
referencia en el mismo gráfico.

### El perfilador no distingue soluciones rápidas en problemas de programa
**Qué pasa:** en los problemas de programa (stdin/stdout), cada ejecución incluye el tiempo de
arranque del lenguaje, que con los tamaños de entrada del perfil suele ser mayor que el
trabajo en sí. El perfilador indica entonces que el resultado no es fiable, o que la solución
es "demasiado rápido para distinguirla aquí". Muestra con claridad las soluciones lentas (por
ejemplo O(n²)), pero no puede separar O(n) de O(n log n).
**Solución:** ninguna; un resultado "demasiado rápido" significa que la solución es
suficientemente rápida para el problema.

### "Buscar una entrada más pequeña que falle" es solo para problemas de función
**Qué pasa:** el reductor no está disponible en problemas de programa ni de SQL; la app
explica por qué. Solo funciona con fallos cuya entrada ya puedes ver (pruebas públicas,
pruebas personalizadas y la primera prueba oculta que falla), se detiene tras 60 ejecuciones
o 30 segundos, y usa las comprobaciones de entrada del problema solo si Python está instalado.
**Solución:** en un problema de programa, recorta a mano la entrada que falla y agrégala como
prueba personalizada.

### Los problemas de SQL tienen menos herramientas
**Qué pasa:** los problemas de SQL no tienen pruebas personalizadas, depurador, perfilador de
complejidad, reductor ni servidor de lenguaje. Las entrevistas simuladas nunca eligen
problemas de SQL.
**Solución:** usa Ejecutar con las pruebas públicas y compara las tablas de resultados.

## Aprender

### Límites de tamaño de entrada en los temas de Aprender
**Qué pasa:** cada tema limita la entrada que puedes escribir para que la animación se pueda
seguir. Los temas de búsqueda de caminos en cuadrículas aceptan cuadrículas de hasta 12 × 16,
el tema del viajante con máscara de bits hasta 5 ciudades, y una animación tiene como máximo
2000 pasos. Una entrada más grande muestra un error en lugar de ejecutarse.
**Solución:** usa una entrada más pequeña o uno de los ejemplos predefinidos.

### Los ejemplos de Aprender están solo en Python y TypeScript
**Qué pasa:** el código de ejemplo junto a cada animación está en Python y TypeScript; no hay
ejemplos en PHP ni en Rust.
**Solución:** ninguna.
