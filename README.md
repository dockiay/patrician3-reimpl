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
  Si tu copia es española, mira «¿Qué copia del juego sirve?» más abajo.
  - `Patrician3.exe`: 2.961.408 bytes,
    SHA-256 `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`
  - `p2arch0_eng.cpr` de la misma instalación.
- Rust 1.97 (lo instala `rustup` solo, por `reimpl/rust-toolchain.toml`).
- Windows 10/11 para el cliente nativo.

## Jugar (la manera fácil)

> **¿Has bajado el código con «Code > Download ZIP»?** También vale: descomprime
> y haz doble clic en **Jugar**. Se descarga solo el juego compilado y sigue en
> el paso 3.

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

## ¿Qué copia del juego sirve?

El programa necesita el **`Patrician3.exe` en inglés, versión 1.1**: el código
está hecho sobre ese ejecutable y con otro no arranca. La ventana de «Jugar» lo
comprueba y te avisa en rojo si eliges otra edición.

| Tienes... | ¿Sirve? |
|---|---|
| Patrician III de **GOG** | Sí. Trae el inglés 1.1. |
| Edición **española** de caja (u otro idioma) | Sola no. Hace falta además el `Patrician3.exe` inglés 1.1 y su `p2arch0_eng.cpr`. |
| Otras versiones (1.0, Gold, Steam...) | No probadas; lo más probable es que no arranquen. |

## Jugar en español (u otro idioma)

El juego siempre arranca con el ejecutable inglés. El español se le añade
desde tu copia española con dos archivos que se quedan en tu equipo:

1. **`p2arch0_spa.cpr`**: está en la carpeta de tu juego español. Cópialo junto
   a `p3-juego.exe`. Con él salen en español misiones, nombres, cartas y tutorial.
2. **`idioma\spa.pack`**: los menús y mensajes, que en el original están dentro
   del ejecutable. Se genera una vez comparando tus dos ejecutables:
   - Instala [Python](https://www.python.org/downloads/) (marca «Add python.exe to PATH»).
   - En la carpeta del código, abre una consola (clic derecho > «Abrir en Terminal») y escribe:
     ```
     pip install -r config/reimpl-requirements.lock.txt
     python scripts/gen_language_pack.py "RUTA_INGLESA\Patrician3.exe" "RUTA_ESPAÑOLA\Patrician3.exe" spa idioma\spa.pack
     ```
   - Copia la carpeta `idioma` junto a `p3-juego.exe`.
3. Abre el juego, ve a **Opciones** y cambia «Language» a Español. Se aplica al
   reiniciar el juego.

Sin el paso 2 el juego funciona igual, con los menús en inglés y el resto en
español. Detalle técnico en `reimpl/IDIOMA.md`.

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
