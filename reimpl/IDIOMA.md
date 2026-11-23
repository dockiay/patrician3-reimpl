# Idioma del juego (Opciones → Language)

Cada EXE lleva su idioma fijo: el inglés pide `p2arch%d_eng.cpr` y `scripts/*_eng.*`, el español `_spa`. Las cadenas de
interfaz están dentro del EXE y el código elevado de `ui_rt` está atado a las direcciones del EXE inglés (el español tiene otro
`.text`), así que **el cliente cambia el idioma del CPR, no el del EXE**:

- Misiones, nombres de ciudades y barcos, créditos y tutorial salen en el idioma elegido.
- Los menús y ventanas siguen en inglés (sus cadenas viven en el EXE). Traducirlos exigiría mapear las cadenas del EXE español
  al inglés o re-elevar el EXE español: trabajo aparte.

## Uso
- **Opciones** (menú principal o en partida): fila «Language»/«Idioma» debajo de «Autosave», igual que las demás (etiqueta a la
  derecha, flechas doradas de `images/options/Kugel_komplett.tga`, valor centrado con el nombre del idioma en su lengua). Para
  hacerle sitio, al abrir la ventana se suben 4 px las 13 filas del original y se juntan 2 px por fila (posición del hijo y su
  entrada en la lista de hijos de la ventana, que es la que usan el dibujo y los clics; `ui_rt/language.rs`). Se guarda en
  `idioma.txt` (carpeta del juego) y **se aplica al reiniciar el juego** (el valor lo indica: «(al reiniciar)»).
  Pruebas: `ui_language_options` (menú) y `ui_language_options_game` (en partida, con `P3_AUTOSAVE_PAT`).
- `p3-juego menu EXE CPR --idioma spa` fuerza un idioma (sin guardar).
- Idiomas disponibles: los `p2arch0_<código>.cpr` de la carpeta del CPR, de `--juego` y de la del EXE (`eng` primero).
  Con un solo idioma la fila no aparece.

## Cómo funciona
`p3_client::idioma::effective_cpr` genera una vez `idioma/p2arch0_<código>.cpr` en la carpeta del juego: entradas del CPR del
idioma (con `_spa.` → `_eng.`, que es lo que pide el EXE) más las que solo tiene el inglés (fuentes elgreco/tiepolo/msgothic…).
Se regenera si cambia el tamaño o la fecha de alguno de los dos CPR (≈ 250 MB, unos segundos). Nunca va a Git.
Selector: `p3-mvp/src/ui_rt/language.rs` (superpone la fila en `Runtime::present`, la ventana se detecta con los ganchos de
`video_options.rs`).

## Pruebas
- `cargo test -p p3-client --lib idioma` (sintético; con `P3_IDIOMA_DIR=<carpeta con eng y spa>` también con los reales).
- `P3_EXE=… P3_CPR=… cargo test -p p3-mvp --test ui_language_options` (clic en `>`, escribe `idioma.txt`, captura en
  `P3_UI_CAPTURE_DIR`). Con el CPR español derivado el menú y Opciones arrancan igual.
- Pendiente: probar en Windows con partida entera y ver el texto de misiones/tutorial.

## Menús y mensajes en español (14/11, paquete de idioma)

- `scripts/gen_language_pack.py <Patrician3.exe inglés> <Patrician3.exe español> spa <salida>` alinea las cadenas de ambos EXE (anclas de
  código, cadenas idénticas, LIS y Needleman-Wunsch) y escribe `idioma/spa.pack` (`S` cadena, `P` tabla, `X` caracteres extra). Son datos
  comerciales: **no van a Git**; se genera en local y p3-juego lo busca en `idioma/<código>.pack` (carpeta del juego, de los EXE o del CPR).
- El tiempo de ejecución traduce en los consumidores de cadenas C (printf, strcpy/strcat, DrawText...) por la dirección del literal inglés.
- `P` parchea la tabla de resaltado del menú (0x6970D8 en el EXE inglés, 0x696978 en el español): evita las letras fantasma.
- ñ Ñ ¿ ¡: el CPR derivado alarga cada tira de fuente con cuatro celdas (virgulilla de `~`, `?`/`!` girados) y la línea `X` añade esos
  caracteres al juego de caracteres de las fuentes. Sello del CPR derivado `v2`.
- Límites: 3.316 cadenas aplicadas, 95 descartadas; las no emparejadas siguen en inglés. Sin prueba en el exe de Windows.

## Web y streaming (14/11, mismo idioma que p3-juego)

- Lógica común en `p3-mvp/src/lang_cpr.rs` (`plan`: de dónde sale cada entrada del CPR combinado, fuentes con ñ ¿ ¡). `p3-juego`
  escribe el resultado en disco como antes (mismo archivo byte a byte); `p3_client::idioma::{prepare, apply}` hacen lo que hacía
  `p3-juego menu` (CPR, paquete `idioma/<código>.pack`, selector) y los usa también `p3-stream`.
- **p3-stream**: `--idioma CÓDIGO`, `--idiomas CARPETA` (más sitios con `p2arch0_<código>.cpr` e `idioma/*.pack`). `Servir-Stream.ps1`
  pasa `D:\AI\P3-Prueba` (o `P3_IDIOMAS`) y la primera vez copia su `idioma.txt` a `partidas\`. El CPR derivado se escribe una vez en
  `partidas\idioma\`. La elección de Opciones se aplica al reiniciar el servidor.
- **p3-web**: la página copia todos los `p2arch0_<código>.cpr` e `idioma/*.pack` de la carpeta elegida (o de «Cargar desde el PC»;
  `serve.py` los ofrece). El CPR combinado no se escribe: `p3_assets::cpr::build_virtual` monta el índice en memoria y lee cada
  entrada del CPR inglés o del español (prueba `cargo test -p p3-web --lib real_spanish` con `P3_IDIOMA_DIR` y
  `P3_IDIOMA_DERIVED`: idéntico al de p3-juego). Elección en `save/idioma.txt` (OPFS); se aplica al volver a pulsar Jugar tras
  recargar la página. Con un solo CPR no hay selector.

### Revisión de traducciones (15/11)

El emparejado inicial por orden desplazaba tablas de palabras cortas y no sabía de módulos con otro orden. Ahora: (1) anclas de código,
(2) Needleman-Wunsch con `sim2` (marcas, longitud, cognados) por huecos, con segunda pasada laxa entre vecinos, (3) `refine`: diccionario
inglés-español aprendido por coocurrencia y reasignación por contenido, (4) `MANUAL_RUNS/TEXT/ADDR` para lo que el EXE español arma de otra
forma («Yes»→«Sí», meses, «Age»...). ~3.400 cadenas. Auditoría de ventanas de edificio: `P3_PACK=... P3_AUDIT_OUT=out.tsv cargo test -p
p3-mvp --test ui_pages translation_audit` (con `P3_EXE`, `P3_CPR` inglés, `P3_SAVE_FILE`). Revisar los textos de otras pantallas con la misma idea.

### Cartas, bocadillos y tablas de punteros (15/11)

- `gen_language_pack.py` alinea también las **tablas de punteros** (listas que el código indexa): por posición, y por contenido si el EXE español ordena distinto las filas (balance de la oficina).
- `load_language_pack` escribe las traducciones en memoria (0x6000_0000) y **reescribe esos punteros** en el EXE cargado (`redirect_slots`, repetido cada 30 fotogramas desde los dibujos de texto): así los bocadillos y textos que el código copia sin pasar por una rutina interceptada salen traducidos.
- Textos que lee el simulador del EXE inglés (cartas, batallas, misiones, informes): `p3_assets::exe::set_translations` y `PeImage::pointed_cstr` / `LetterTextBank::cstr` consultan primero la traducción.
- Requiere recompilar el cliente (nuevo exe) además del `spa.pack`.

### Traducciones vacías (15/11)

- La ficha de viviendas inglesa arma el nombre como «Half-timbered » + «house»/«houses» (tabla 0x6A2440); la española usa «Casas de madera» y deja
  el sufijo vacío (tabla 0x6A23D8). El paquete no podía expresar una traducción vacía y salía «Casas de ladrillohouse», con la cifra encima.
- `S dirección` sin texto es ahora una traducción vacía (cliente y generador, `FORCE_ADDR` 0x6A22BC/0x6A22C4). Requiere cliente nuevo y spa.pack
  regenerado; un cliente viejo ignora esas líneas.

### Frases con los argumentos en otro orden (17/11)

- «Misión aceptada» de la taberna salía en inglés: «Meet %s %s in the %s in %s by %u. %s %u, in order to deliver %d%s %s.» (0x69E748, por
  `CString::Format` 0x64DF80). El EXE español pone la fecha delante («Reúnete antes del %u de %s de %u con...»), así que las marcas no coinciden
  y el generador la descartaba.
- `ui_rt/crt.rs` admite ahora argumentos posicionales base 1, `%$N$s` (la grafía del formateador propio del juego, 0x64D410) y `%N$s`.
  Así ya funcionan también 0x6A8B10 y 0x6AE8B0, que el paquete traía con `%$N$` y por `CString::Format` salían con el `%$` a la vista.
- `MANUAL_SENTENCES` añade 0x69E748 (posicional) y cinco cadenas que quedaban sin par: «Administrator (stage %d)», «Navigate manually»,
  «on %i. %s %i in %s», «%i trading offices». Revisadas las 237 cadenas del EXE inglés sin traducción: el resto son depuración en alemán,
  rutas o formatos iguales en los dos idiomas.
- spa.pack regenerado (+5 líneas). Requiere cliente nuevo: el viejo mostraría los `%$N$` sin sustituir.
