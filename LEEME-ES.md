# ROM Auto Extract 1.0.0 — versión de prueba para Windows x64

Este paquete contiene el plugin y una compilación compatible de RecompLauncher.
El lanzador oficial beta.67/beta.68 no ofrece extracción local ni selección de
ROMs a los plugins. Por eso el archivo .rlplugin solo funciona con la integración
nativa rom.unpack incluida en este ejecutable. No es una versión oficial.

## Usarlo con tu biblioteca actual

1. Cerrá RecompLauncher por completo, incluida su instancia en la bandeja.
2. Como respaldo, copiá la carpeta de datos que muestra Configuración → General
   en tu lanzador habitual antes de usar esta compilación de prueba.
3. Ejecutá recomplauncher.exe de ESTA carpeta. Usa la identidad y la carpeta de
   datos habituales del lanzador. El ejecutable original no fue reemplazado.
4. En Configuración → Plugins → Instalar desde archivo, elegí
   rom-auto-extract.rlplugin de esta carpeta y dejalo activado.
5. Abrí 2 Ship 2 Harkinian y continuá con la copia comprimida detectada. También
   podés seleccionar el ZIP, RAR o 7z desde la preparación del juego.
6. Aparecen «Extrayendo…» y «Seleccionando…». Se conserva el comprimido y se
   selecciona la ROM extraída junto a él para continuar la preparación normal.

La página del plugin debe mostrar «Activo». No necesita permanecer abierta,
tampoco cuando jugás en Big Picture. Se desactiva desde Configuración → Plugins.
Una actualización oficial del lanzador puede quitar esta integración: conservá
este paquete hasta que el soporte se incorpore al proyecto oficial.

Es una compilación de desarrollo sin optimizar. Puede mostrar una consola aparte.
Requiere Windows x64, WebView2 y el runtime de Visual C++ que usa el lanzador.

## Qué comprueba

- Extrae localmente ZIP, RAR y 7z; no descarga ROMs.
- Aplica las comprobaciones declaradas por el juego, incluida la cabecera N64
  cuando corresponde. La extracción no convierte una región o revisión incorrecta
  en una copia compatible; después continúa la verificación normal del juego.
- Admite una ROM o imagen de disco de un solo archivo por comprimido. Si hay
  varias ROMs que pasan las comprobaciones, pide resolver esa ambigüedad.
- Deja la ROM en la misma carpeta que el comprimido, aunque dentro del comprimido
  estuviera en una subcarpeta. Conserva el archivo original.
- Nunca sobrescribe un archivo distinto. Una copia existente solo se reutiliza
  si es idéntica byte por byte. Si hay conflicto, renombrá la copia existente.
- Los archivos temporales se limpian al terminar o fallar. No publica ROMs
  incompletas. Usa el lector con límites del lanzador: 16 GiB / 100.000 entradas.
- CUE/BIN con varias pistas, carpetas completas de juego, archivos con contraseña
  y volúmenes divididos requieren preparación manual. No ejecuta archivos del
  comprimido. Los juegos que declaran que leen ZIP conservan su comportamiento.

## Verificación realizada

- 7 pruebas nuevas nativas pasaron: ZIP con subcarpetas, 7z, RAR, copia ya
  seleccionada, permisos/interruptor global, reutilización/conflictos y copias
  ambiguas/inválidas. Los datos usados son sintéticos, sin ROMs comerciales.
- Suite nativa completa: 904 pasaron, 1 omitida, 1 falló en pads::tests::through_sdl.
  Esa prueba esperaba solo un mando Xbox y detectó también un mando PlayStation.
- Interfaz: 720 pruebas pasaron, 11 omitidas, ninguna falló; incluye compatibilidad
  del plugin y ciclo de los mensajes de extracción/selección.
- Ejecutable compilado; paquete .rlplugin inspeccionado y sin errores ZIP.
- Prueba real del 22/11/2026: plugin instalado y activado desde la interfaz.
  2 Ship 2 Harkinian preparó la copia comprimida y arrancó mostrando la
  introducción con la ROM USA. La ROM europea se extrajo correctamente, pero
  el propio juego la rechazó; quedó seleccionada la USA.
- Super Mario 64 Co-op DX y Ship of Harkinian quedaron configurados con las
  ROMs extraídas de ZIP. El usuario confirmó que ambos funcionan correctamente.
- Se observó el estado «Extrayendo…» y la selección persistida del archivo
  descomprimido junto al original. Las pruebas reales cubren ZIP; RAR y 7z
  están cubiertos por pruebas automáticas con datos sintéticos.
- La detección por nombre del lanzador puede elegir una región incompatible.
  En ese caso hay que elegir el comprimido de una región admitida; extraerlo
  no cambia la región ni la revisión del juego.

## Compartirlo

Compartí el ZIP completo ROM-Auto-Extract-1.0.0-Windows.zip: contiene el ejecutable
compatible, el plugin, instrucciones, licencia, avisos y código fuente exacto.
No incluyas tus ROMs, biblioteca, credenciales ni carpeta de datos.

El .rlplugin solo puede compartirse por separado con personas que ya tengan una
versión del lanzador con rom.unpack. Para que sea instalable en el lanzador
oficial sin este ejecutable, el mantenedor debe incorporar primero los cambios
nativos. El código y la documentación están preparados para proponerlos al
repositorio original; no se publicó ni se envió nada a terceros.

Licencia del lanzador y de esta distribución: GPL-3.0-or-later. Conservá LICENSE,
CREDITS.md y el código fuente correspondiente al redistribuir el ejecutable.
El código fuente está en source-rom-auto-extract.zip. Para reconstruirlo, instalá
Rust estable y las herramientas C++ de Visual Studio y ejecutá desde el código:

    powershell -ExecutionPolicy Bypass -File tools/build-rom-auto-extract.ps1

El script usa cargo y genera una compilación de desarrollo y el .rlplugin.
