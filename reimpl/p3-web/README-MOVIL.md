# Probar p3-web desde el móvil (privado, https)

`SharedArrayBuffer` exige contexto seguro: http por IP local no vale. La opción privada y con certificado de confianza
es **Tailscale** (gratis, VPN personal): `tailscale serve` da `https://TU-PC.tu-tailnet.ts.net` solo a tus dispositivos. Nada va a internet
(no usar `tailscale funnel`: eso sí publica).

1. PC: instalar Tailscale (`winget install Tailscale.Tailscale`) e iniciar sesión. iPhone: app **Tailscale de la App Store** (VPN activada), misma cuenta.
2. Una vez, en https://login.tailscale.com/admin/dns activar **MagicDNS** y **HTTPS Certificates**.
3. PC: doble clic en `Servir-Movil.cmd` (compila el `.wasm` la primera vez; escribe la URL https que da `tailscale serve`).
4. Móvil (con Tailscale activo): abrir esa URL en Chrome → **Cargar desde el PC** (descarga `Patrician3.exe` y el `.cpr` de la carpeta
   `P3_GAME_DIR`, por defecto `D:\AI\Patrician3\games\lab-english`, a la memoria privada del navegador; ~240 MB, una sola vez) → **Jugar**.
5. Otra carpeta de juego: `set P3_GAME_DIR=D:\ruta` antes de lanzar el `.cmd`. En el PC puedes usar «Elegir carpeta del juego» en vez de lo anterior.

Parar: cerrar la ventana del servidor y `tailscale serve reset`.

## iPhone (Safari; en iOS todos los navegadores son WebKit)
- Abrir la URL en **Safari**. Requisitos: iOS 16.4+ (SharedArrayBuffer con COOP/COEP, OPFS con acceso síncrono, OffscreenCanvas, Worker de módulo).
  El `.wasm` solo usa bulk-memory, multivalue, sign-ext y nontrapping-fptoint (sin SIMD, hilos ni excepciones): lo soporta Safari.
- No hay pantalla completa en iPhone: Compartir → **Añadir a pantalla de inicio** y abrir desde el icono (modo app, sin barra). Además Safari
  puede borrar los datos de un sitio sin uso tras ~7 días; instalado en la pantalla de inicio no lo hace. Si se borran, pulsar de nuevo «Cargar desde el PC».
- Un toque = clic izquierdo; marcar «Toque = botón derecho» para clic derecho. Sin teclado físico no hay teclas.
- **Sin probar en un iPhone real** (no hay dispositivo en el entorno). Riesgo principal: memoria (el motor reserva 256 MB y el CPR de 237 MB va entero en memoria WASM, ~0,5 GB en total); si Safari recarga la pestaña, es eso.
