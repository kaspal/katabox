# KataBox

**Práctica de programación sin conexión, en tu escritorio.** Resuelve problemas de fácil a difícil en Python, JavaScript/TypeScript, PHP y Rust, con un editor real, pruebas públicas y ocultas, pistas, soluciones explicadas y un depurador. Sin cuenta, sin suscripción y sin internet después de la instalación.

[Read in English](README.md)

> **Estado:** desarrollo temprano (pre-alfa). Los instaladores para Windows llegan con la primera versión.

## Funciones planeadas

- Un catálogo creciente de problemas originales en ~20 patrones (arreglos, dos punteros, ventana deslizante, árboles, grafos, programación dinámica…)
- Ejemplos públicos, tus propias pruebas (la salida esperada se calcula por ti) y pruebas ocultas que muestran la primera entrada que falla
- Editor con autocompletado y diagnósticos; depurador paso a paso con "Depurar este caso"
- Pistas por niveles, explicaciones y soluciones de referencia en cada lenguaje
- Modo entrevista simulada, repaso espaciado, problema del día, listas personales
- Perfilador de complejidad y visualizador de estructuras de datos
- Tutor de IA local opcional con Ollama (nada sale de tu computador)
- Interfaz en inglés y español

## Instalación

**Windows 10/11 (x64):** descarga `KataBox_<versión>_x64-setup.exe` desde la página de [Releases](https://github.com/kaspal/katabox/releases) y ejecútalo.

- Se instala solo para tu usuario; no hace falta ser administrador. También hay un `.msi` para equipos administrados.
- Mientras los instaladores no estén firmados, Windows SmartScreen puede avisar al abrirlo por primera vez: elige **Más información → Ejecutar de todas formas**.
- Al iniciarse por primera vez, KataBox pregunta qué lenguajes instalar y los descarga (Python ocupa unos 100 MB). Después funciona sin conexión.
- **Desinstala** desde Configuración → Aplicaciones. Tu progreso se conserva salvo que marques **Eliminar los datos de la aplicación**.

Los paquetes para Linux (.deb, .rpm, AppImage) salen del mismo flujo de publicación; consulta [docs/packaging.md](docs/packaging.md) para sus requisitos.

## Licencia

Código: [MIT](LICENSE-MIT) o [Apache-2.0](LICENSE-APACHE), a tu elección. Contenido de problemas en `content/`: [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/deed.es).
