# Patrician III: reimplementación en Rust (vanilla 1.0)

Reimplementación del motor de **Patrician III** (Ascaron, 2003) en Rust: la
simulación, el cliente a pantalla completa, el sonido, las partidas `.pat` y la
red. Es la versión **vanilla**: el juego tal como era, con arreglos de fallos,
sin funciones nuevas.

**Este repositorio no incluye ningún archivo del juego original**: ni el
ejecutable, ni el archivo de recursos `.cpr`, ni gráficos, sonido, música,
vídeos, textos ni partidas. Para jugar necesitas **tu propia copia comprada**
del juego; el programa lee esos archivos de tu instalación cada vez que arranca.

Buena parte del código se obtuvo traduciendo funciones del ejecutable original
a Rust con las herramientas de `reimpl/tools`, para que el juego se comporte
igual que el original (paridad comprobada en emulación).

Proyecto no oficial, sin relación con Ascaron, Kalypso Media ni los dueños de la
marca Patrician.

## Qué necesitas

- Patrician III **en inglés, versión 1.1** (la que vende GOG). Es la única
  edición soportada: el código trabaja con las direcciones de ese ejecutable.
  - `Patrician3.exe`: 2.961.408 bytes,
    SHA-256 `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`
  - `p2arch0_eng.cpr` de la misma instalación.
- Rust 1.97 (lo instala `rustup` solo, por `reimpl/rust-toolchain.toml`).
- Windows 10/11 para el cliente nativo.

## Jugar (la manera fácil)

1. Descarga `Patrician3-vanilla-1.0-windows.zip` de la
   [página de versiones](https://github.com/dockiay/patrician3-reimpl/releases/latest)
   y descomprímelo en una carpeta cualquiera.
2. Doble clic en **Jugar**.
3. La primera vez sale una ventana que busca tu copia del juego. Si no la
   encuentra, pulsa «Buscar...» y elige la carpeta donde la instalaste. Te dice
   si la carpeta es correcta.
4. Pulsa «Jugar». Las siguientes veces se abre directamente.

No se cambia nada de la carpeta de tu juego; las partidas se guardan junto al
programa, en `save`. Como el programa no está firmado, Windows puede avisar
la primera vez: «Más información» y «Ejecutar de todas formas».

## Compilarlo tú

Solo si quieres tocar el código. Con Rust instalado ([rustup.rs](https://rustup.rs)),
doble clic en `Compilar.cmd`: compila y deja la carpeta `jugar` lista con el
mismo asistente. A mano:

```
cd reimpl
cargo build --release -p p3-client --bin p3-juego
```

El programa queda en `reimpl/target/release/p3-juego.exe`. Sin el asistente,
busca `Patrician3.exe` y `p2arch0_eng.cpr` a su lado, o las rutas de un
`p3-juego.ini` (ejemplo en `reimpl/p3-client/p3-juego.ini.example`). Desde la
línea de órdenes:

```
p3-juego menu "RUTA\Patrician3.exe" "RUTA\p2arch0_eng.cpr" [--ventana] [--moderna 1920x1080]
```

### Otros idiomas

Los menús salen en inglés. Si además tienes el juego en otro idioma (por
ejemplo el español), puedes generar el paquete de idioma desde tus dos
ejecutables; el resultado son datos del juego y se queda en tu equipo:

```
pip install -r config/reimpl-requirements.lock.txt
python scripts/gen_language_pack.py Patrician3_ingles.exe Patrician3_espanol.exe spa idioma/spa.pack
```

Deja `idioma/spa.pack` y el `p2arch0_spa.cpr` de tu copia junto al juego y elige
el idioma en Opciones (se aplica al reiniciar). Detalle en `reimpl/IDIOMA.md`.

## Estructura

| Carpeta | Contenido |
|---|---|
| `reimpl/p3-core` | Modelo de memoria, CPU x86 de apoyo y lógica del juego |
| `reimpl/p3-sim` | Simulación sin pantalla (ciudades, comerciantes, IA, batallas) |
| `reimpl/p3-mvp` | Interfaz del juego y enlazador de ventanas (`ui_rt`) |
| `reimpl/p3-render` | Dibujo de ciudad, carta y paneles |
| `reimpl/p3-audio` | Sonido y música (sustituye a Miles) |
| `reimpl/p3-assets` | Lectura del `.cpr` y formatos gráficos |
| `reimpl/p3-save` / `p3-state` | Partidas `.pat` |
| `reimpl/p3-net` | Red |
| `reimpl/p3-client` | Cliente nativo `p3-juego` |
| `reimpl/p3-web`, `p3-wasm`, `p3-stream`, `web` | Versión de navegador (experimental) |
| `reimpl/tools` | Herramientas que generan código a partir de **tu** ejecutable |

## Pruebas

Las pruebas de paridad comparan con el juego original y usan datos sacados de
él, así que no están en este repositorio. `cargo build` y `cargo check` funcionan
sin ellas; `cargo test` no compila.

## Licencia

El código propio va con licencia MIT (`LICENSE`). Esa licencia no cubre nada de
Patrician III.
