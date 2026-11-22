'use strict';
(async () => {
  const status = document.getElementById('status');
  try {
    const info = await rl.info();
    const supported = Array.isArray(info.features) && info.features.includes('rom.unpack');
    status.textContent = supported
      ? 'Activo: las ROMs comprimidas se extraen durante la preparación del juego.'
      : 'Esta versión del lanzador necesita la integración rom.unpack incluida con este plugin.';
    status.dataset.state = supported ? 'ready' : 'unsupported';
  } catch (error) {
    status.textContent = 'No se pudo comprobar la integración: ' + String(error.message || error);
    status.dataset.state = 'error';
  }
})();
