# Dependencias del decoder MP3

El código propio conserva MIT. Desde el bloque16, `p3-audio` enlaza
**Symphonia0.6.1** con las features `mp3`, `id3v1`, `id3v2`; resto de defaults
desactivado. No sustituye el decoder IMA propio ni añade servicios del sistema.

Symphonia es de Philip Deljanov y The Project Symphonia Developers,
copyright2019–2026, **MPL-2.0**. Se usa sin modificaciones, por Cargo; no se copia
su implementación al código propio. [Repositorio](https://github.com/pdeljanov/Symphonia),
[release0.6.1](https://github.com/pdeljanov/Symphonia/releases/tag/v0.6.1) y
[licencia conservada](licenses/Symphonia-MPL-2.0.txt).

Versiones nuevas fijadas por Cargo.lock y licencias declaradas por sus paquetes:

| Paquete | Versión | Licencia declarada |
|---|---|---|
| symphonia / symphonia-core / symphonia-metadata / symphonia-bundle-mp3 | 0.6.1 | MPL-2.0 |
| autocfg | 1.5.1 | Apache-2.0 OR MIT |
| bitflags | 2.13.2 | MIT OR Apache-2.0 |
| bytemuck | 1.25.2 | Zlib OR Apache-2.0 OR MIT |
| lazy_static | 1.5.1 | MIT OR Apache-2.0 |
| log | 0.4.34 | MIT OR Apache-2.0 |
| num-complex | 0.4.6 | MIT OR Apache-2.0 |
| num-traits | 0.2.19 | MIT OR Apache-2.0 |
| regex-lite | 0.1.9 | MIT OR Apache-2.0 |
| smallvec | 1.16.4 | MIT OR Apache-2.0 |

Las fuentes, checksums y licencias originales están también en la caché Cargo
local ignorada; las versiones previas del workspace no cambian. No se publica
un binario de distribución en este bloque. Su empaquetado debe conservar los
avisos y licencias de las dependencias que incluya.

FFmpeg7.1 es solo herramienta de referencia externa ya inventariada; no se
invoca desde la crate ni es dependencia de ejecución del backend. Los fixtures
MP3 sintéticos se codifican a partir de señales propias. Los MP3/PCM comerciales
siguen únicamente en las carpetas ignoradas.
