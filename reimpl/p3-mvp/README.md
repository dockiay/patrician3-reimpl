# Prototipo visual de escritorio

En Windows, ejecutar `MVP-Visual.cmd` desde la raíz. El lanzador compila el
núcleo Rust con el entorno local y abre el cliente. Elegir una partida `.pat`
compatible con S05 y un CPR con los recursos de la misma edición. La primera
compilación tarda más. Los archivos comerciales se leen localmente.

La sesión selecciona inicialmente la primera oficina existente de un comerciante
humano; el selector permite elegir otra oficina humana. Muestra su ciudad,
fecha, oro, población y existencias. Arrastrar el mapa o
utilizar las barras para mover la cámara; seleccionar una mercancía para
comprar o vender. En «Consultar barcos» se puede inspeccionar la carga y
comprar o vender con un barco propio atracado en la ciudad seleccionada; la
mercancía y la cantidad se eligen en la ventana principal. Para barcos se
exigen lotes completos de 200 o 2.000 unidades según la mercancía. Las
cantidades son las unidades internas del original. Desde esa misma ventana se
puede previsualizar una ruta entre puertos sobre la rejilla de navegación del
CPR. «Enviar barco» inicia el viaje de un barco humano suelto en puerto, separa
un miembro subordinado o mueve el convoy si se elige su líder. Un líder o miembro de convoy ya navegando puede recibir un nuevo destino desde su posición actual; el miembro se separa. Durante salida o atraque, el líder puede cambiar de destino o invertir la maniobra; un miembro puede separarse y recibir una orden si está apto para zarpar. Las demás órdenes en marcha aún son limitadas. La sesión admite varios viajes humanos simultáneos; el contraste x86 cubre dos barcos independientes con una ruta común, no todas las interacciones marítimas. Los barcos cambian
de ciudad al terminar el trayecto. Las rutas propias y las compartidas entre
puertos pueden guardarse y reanudarse; se ha probado con travesías de la partida
local, sin validación en el juego.
«Activar flota IA» reconstruye los barcos y convoyes disponibles del guardado y activa las decisiones navales ya portadas. Con esa opción, avanzar detiene el reloj ante los encuentros admitidos entre convoyes o contra un barco suelto de IA. El cliente completa el tick y deja la batalla en pausa. «Avanzar combate» ejecuta un paso sin vista; «Reanudar / pausa» lo avanza con tiempo de pared. Al pausar se procesa el paso encolado y se puede guardar y reabrir. No muestra la escena naval ni reanuda el calendario mundial. La captura, la resolución final, los encuentros con jugador humano y otras variantes aún incompletas pueden detener el avance con un error. La ruta completa no tiene paridad acreditada en partida.
El tiempo puede avanzar por días, paso a paso o de forma continua. Guardar
crea una copia nueva: no sobrescribe archivos existentes. Para retomar, abrir
esa copia y el mismo CPR. Una batalla guardada se abre en pausa. Cerrar con cambios pendientes pide confirmación.
Para los informes de combate, el programa busca el `Patrician3.exe` inglés
compatible junto al CPR. También puede indicarse con `P3_ORIGINAL_EXE`; solo se
leen sus textos tras comprobar su hash, sin ejecutarlo. Sin esos datos se
conserva la batalla si el cierre necesita un informe para un jugador humano.

El terreno se dibuja a partir de `.town`, `.let` y `.aim`. De los registros de
edificios se muestra el sprite cuando la clase gráfica tiene una única hoja en
el CPR, o es una casa o muralla con variante identificable (grado gráfico 2).
Los demás se indican con marcas opcionales. El número dibujado se indica en el
pie. No están resueltas todas las variantes, el orden de profundidad,
sombras ni animación; los barcos se muestran en una lista, sin sprites. El ambiente
de mercado es opcional: se decodifica mediante el puerto IMA de `p3-audio` y
utiliza la salida WAV de Windows. No reproduce aún el mezclador ni los
disparadores de sonido del original.
Las marcas son ayudas para inspeccionar registros, no gráficos originales.

El reloj, las compras, ventas y el guardado usan los módulos de simulación
existentes. Muchos sistemas permanecen incompletos o inactivos, entre ellos la
IA, el combate en el mar, otras órdenes en marcha y otros barcos. La sesión es experimental: sus resultados no
acreditan paridad con el original ni constituyen una partida completa. La
escritura verifica que el simulador pueda volver a leer el resultado, pero no
se ha probado que el juego original lo cargue. Conservar siempre el archivo
de partida de origen.

## Construcción por el jugador (J03)

`build_check {type,row,col}` consulta una casilla del plano (`0x004FD020`, `0x004FB9C0`) y devuelve si se puede, los materiales de ciudad que pide
`0x00521900` y el oro; `build` empieza la obra con el mismo compromiso atómico que el pozo (`Sim::start_building_work`): si el original no la
empezaría, o el puerto no admite alguna dependencia, devuelve error y no cambia nada. En la partida de prueba (`examples/plan_probe.rs`) se empieza el tipo 4 (pabellón de caza), 6–0x1D (talleres y granjas; 0x15–0x1D son viviendas),
0x28 (pozo), 0x29–0x2C (hospital, casa de la moneda, escuela, capilla), 0x2D (monumento) y 0x30 (bombarda del puerto) (etapa de torre/defensa, por la orden 0x22 del planificador de defensa); la obra sobrevive a guardar y recargar. Los objetos de mapa y la barra de progreso (`AddMapObject`, `SetBuildingProgress`)
son solo dibujo del original y se ignoran: la sesión deriva la imagen de los registros del plano. Los nombres de tipo están en `research/systems/ciudades/BUILDING_TYPES.md` (la tabla del ejecutable). Siguen sin poderse empezar el almacén 0x1E (`RecountOffice`), 0x26, 0x01 y 0x2E (el original no las empieza con los materiales de esa ciudad) y
los tipos sin casilla válida en la ciudad de prueba (0, 2, 3, 0x1F–0x25, 0x27, 0x2F, 0x31–0x35). Las casillas que hacen leer fuera de la rejilla al original cuentan como no válidas. En el cliente: elegir tipo en «Construir», pulsar en el mapa y confirmar.
Las filas/columnas son las del plano (`fila = y`, `columna = x` de los registros menos el origen del mapa). Sin validación en el juego original.

Alcance comprobado de la construcción: `examples/plan_probe.rs` recorre todas las oficinas humanas de una partida y todos los tipos 1–0x38. La partida
de prueba solo tiene una oficina humana (Lübeck), así que no hay comprobación en otras ciudades. En Lübeck se empiezan 31 de 56 tipos; de los otros 25, 17 no tienen
casilla válida (edificios únicos o con condiciones de ciudad: iglesia 0x1F, ayuntamiento 0x20, ..., puertas y muralla 0x36–0x38), 4 el original no los empieza (0x01, 0x26, 0x2E y 0x05, que sale
fuera del plano) y 0x1E (almacén) necesita el recuento de oficina (`0x004FF2D0`, portado en `p3-core::economy::office_recount` pero que exige un gestor de negocios que la sesión no tiene).

## Animaciones de la ciudad (J02)

`render` acepta `clock_ms` (reloj de animación en milisegundos). El motor avanza los animadores de los objetos (puertas, molinos, baños...) desde 0 en pasos
de 100 ms con un `rand` determinista sembrado por la ciudad; la respuesta incluye `animators`, `animations_shown`, `flame_groups` y `animations`
(casilla, hoja, clase y fotograma visible). La casilla «Animar la ciudad» del cliente repite `render` sumando 2 s al reloj. Límites y
detalle en `research/systems/mapa-dibujo/J02-objetos-animacion.md`. Las figuras de la ciudad y los carros del transporte por tierra se dibujan aparte (ver «Transporte por tierra»).

## Mapa del mundo

El botón «Mapa del mundo» del cliente pide `render_world`: el motor decodifica
`images/vollansichtskarte.bmp` (2400×1800, 8 bits) del CPR con `p3_render::decode_bmp`,
lo escribe como `mundo.png` junto a la imagen de la ciudad y devuelve las ciudades
(nombre del comentario de cada sección de `scripts/StadtDaten.ini`, símbolo, rótulo,
lado del texto y punto de aproximación) y todos los barcos del guardado en el mismo
espacio de píxeles (`+0x1E/+0x22`). La ciudad actual y los barcos propios se resaltan.
Es solo lectura. No se versiona ninguna imagen comercial. Límite: los barcos son
puntos, sin sprites ni rumbo.

## Carta náutica (J06)

El botón «Carta náutica» abre la vista de navegación desplazable. Cada fotograma pide
`{"command":"navmap","width","height","clock_ms","keys":["left","up","right","down"],"mouse":[x,y],"center":[x,y],"ticks":n}`
y el motor (`src/navmap.rs`, sobre `p3_render::navmap`) hace lo de `nav_map_update` (0x0044DCE0):

- desplazamiento con flechas o ratón en el borde (una pantalla cada 1500 ms, tope de 0x80
  píxeles por fotograma, recorte al fondo de 2400×1800; inicio en 810/950) y centrado
  (`0x0044F490`, doble clic o arrastre en el cliente), con paridad x86;
- entidades de tiempo del gestor naval (`0x459380`): tormentas y calmas con su sonido
  (`p3_audio::map_ambient`, clase 3) y dibujo animado (`0x0044B970`, paridad x86);
- lista de barcos visibles (`0x004516B0`, sonido de clase 4) y barcos en el mar con su
  casco, rumbo, flecha y número del convoy (`0x0044CBF0`, paridad x86);
- fondo `Vollansichtskarte.bmp` y texturas reales de `scripts/textures.ini` (`TEX16036`,
  `TEX30000..30008`, flecha `TEX30039` con clave magenta).

Escribe `carta.ppm` junto a la imagen de la ciudad y devuelve desplazamiento, minimapa,
barcos y ciudades visibles (en píxeles de la vista), iconos de tiempo, sonidos
(`carta_*.wav`, con volumen y panorámica), ajustes y `stop_all`. Con `ticks` avanza el
reloj antes de dibujar (`Session::advance_ticks`, mismas guardas que `advance`); los barcos
ajenos solo navegan tras «Activar flota IA». La imagen naval es la viva de la navegación o,
si no hay, la del guardado de la sesión, rehecha cuando cambia el tick.

Avistamientos (C10, sin tocar): la carta dibuja lo que admite `0x004516B0`, que lee
`ship+0x50`/`+0x52` de los barcos propios (contactos). Cuando el tick naval de C10 escriba
esos campos en la imagen naval, los barcos avistados aparecerán sin más cambios
(`p3_render::navmap::World::ships`).

El fotograma se compone con `nav_map_draw` (`0x0044AB70`, orden del original): fondo, marcas,
icebergs (`0x0044B630`), contactos que se desvanecen (`0x004530B0`), barcos y tiempo. Cada
copia mezcla como `sgl_DrawBitmapRect` del controlador `ddraw_Dll.dll` en memoria
(`p3_render::sgl`, paridad con la DLL en emulación, variantes SSE): modo de
`sgl_SetBlendMode`, color e indicadores que `0x004B3610` deriva de `textures.ini`
(`UseColorKey`, `UseAlpha`, `UseFiltering`, `BlendMode`). El cliente reajusta volumen y
panorámica de las muestras que siguen sonando (`adjust`, por id).

En `p3-render` están portados además, con paridad x86, la actualización completa
`0x0044DCE0`, rótulos `0x00452600`, símbolos `0x0044BCC0`, estado `0x0044BFD0`, barcos en
puerto `0x0044C3B0`, rutas `0x0044B330`, copiado incremental `0x004538E0`, animaciones
`0x00458050`, arrastre, toque de ruta, ayudas de iconos, días de viaje, vista previa y envío de
rutas (`0x00451EB0`, `0x0045A6B0`, grupos de problemas `0x0045B9C0`); ver
`reimpl/functions.tsv` (J06).

Datos de la sesión (`navmap::frame_data`): la selección (`selection`, ids de barcos propios;
en el cliente clic sobre un barco propio, Mayús para añadir, Esc para soltar), las rutas
(navegación viva, rutas guardadas propias o compartidas con un planificador propio de
`foundation_navigation`), las oficinas del comerciante (tabla `+0x28CC`, `0x00548230`), las
noticias por ciudad (`0x00535300`, paridad x86) y `AmFluss`/alineación de `StadtDaten.ini`.
Con eso se dibujan rutas, marcas, símbolos, estado y barcos en puerto. Los rótulos usan la
fuente del juego (`fonts.ini` + `.aim`, `p3_render::sgl_font`, paridad con la DLL); la capa
«Nombres (fuente del sistema)» del cliente queda apagada por defecto.

Avistamientos: `navmap::spot_frame` ejecuta en cada fotograma la pasada real de los humanos
(`p3_core::ships::spotting`, `0x0050DC50`/`0x0050D2B0`, paridad x86) sobre la copia que se dibuja;
las ranuras `ship+0x50`/`+0x52` se conservan en el estado de la carta y la simulación no cambia.
Los contactos perdidos se dan de alta en `0x00452F60`, los sonidos de clase 2 entran en la escena del
mapa y la respuesta lleva `sightings.lost/pulses` (el pulso `0x0046F340` no se dibuja aún). El
original la ejecuta dos veces por paquete de reloj `0xC4`; aquí, una por fotograma.

Mezcla SGL (`p3_render::sgl`, paridad con `ddraw_Dll.dll`, variantes SSE que instala el
arranque `0x05028BD8` en cualquier CPU actual): copias normales, texturas de 8 bits con paleta
(rutinas 11–21), escalado al vecino más cercano y filtrado bilineal al ampliar
(`UseFiltering`, rutinas 0, 8 y 9/10).

Las rutas se trazan con el rasterizador de líneas de la DLL (`sgl_DrawLine` → `0x05015080`/`0x05091154`,
`p3_render::sgl::draw_line`, 4.000 casos) y el escalado recorta como el original (2.000 casos).
Límites: una sola región de recorte por línea (la vista); falta el camino Direct3D (`0x05022600`, su resultado depende de la
tarjeta). Se redibuja la vista entera en cada fotograma. Probado en Windows (10/11, PC de Sergio, rustc 1.99.1 MSVC, Python 3.14): compila, pasan
`navmap_cpr`, `sgl_blit` y `sgl_font`, y el cliente arranca sin errores; la ventana con la carta
abierta se comprobó con una captura de Sergio (rótulos con la fuente del juego, marcas de ciudad, iconos y barco elegido). Prueba con recursos reales: `cargo test -p p3-mvp --test navmap_cpr`
(`P3_CPR`, `P3_NAVMAP_SAVE`; `P3_NAVMAP_KEEP` es la base del nombre de las imágenes, no una carpeta; inyecta una tormenta y comprueba dibujo y sonido).
Sin `P3_CRT_STATE` el programa siembra el CRT con el reloj al abrir, como el original al cargar (`srand(timeGetTime())`, llamada en 0x00547F44); el mapa de navegación que piden tormentas e icebergs (tarea 0x0D) se carga del CPR al pasar el primer día.

## Transporte por tierra

«Transporte por tierra» abre la ventana del almacén de la oficina elegida. Solo hay camino entre las parejas de ciudades de
0x672BF8 cuya ciudad tiene la bandera `+0x76C & 0x8000` (`0x00525630`); en las partidas de prueba, Lübeck y Hamburgo. Se eligen
cargas enteras por mercancía (2.000 unidades a granel, 200 en barriles) y la escolta; la ventana recalcula carros, valor, escolta
exigida y su coste (50/200/800 por cada diez carros) como el original. «Enviar» ejecuta `0x005EB3C0` (emboscada y pérdidas si la
escolta es menor que la exigida) y una operación 0x3A por mercancía (`0x0053A2C0`), que crea la tarea 0x0C y la despacha en el acto:
salen las existencias de la oficina y se paga la escolta. Mientras el carro sale (fase 1) o llega (fase 6) se dibuja en la vista de
la ciudad: las figuras de carro salen de las tareas 0x0C del guardado (`Session::land_transport_carts`, como `0x004D8F00`), con la
posición del símbolo de la otra ciudad. La lista muestra los transportes en curso; el viaje continúa con el reloj, sobrevive a guardar
y recargar y termina en unos días (`tests/land_transport_session.rs`: tres días en TEST25). Paridad x86: ventana, envío, operación y
ciudad unida (`p3-core/tests/x86_land_transport_send.rs`, 4.500 casos) y la operación con el despachador real sobre tres partidas
(`p3-sim` `land_transport_start_live`, 18 casos). La tirada de la emboscada usa el CRT compartido de la sesión.

Órdenes JSON: `land_transport_quote {loads:[20], escort}`, `land_transport_send {loads:[20], escort}` y `land_transports`.

## Edificios de la ciudad

Los edificios se colocan en la capa 1 del mapa con las rutinas del original (`0x0058DB40`: vaciar la huella `0x0058DF80` y estampar `0x0058E180`, con paridad x86 en `p3-render/src/scene_grid.rs`); el dibujo recorre las celdas, así que la profundidad y los solapes (`b3`) son los de la cuadrícula.

Además de las hojas únicas, la ciudad dibuja con la regla del original (`0x005928A0`, claves de `0x00521310`)
la iglesia (clase 4: estilo = `Region` de StadtDaten.ini, fase = `Town+0x7A3`), las torres de artillería (41:
estilo = `Region`, variante 0), las puertas de muralla (47: tres piezas separadas dos pasos de rejilla según
`0x0058B740`, estilo en matriz[3], orientación 32/224) y el ayuntamiento (3: estilo = `Region`, **regla inferida**:
el constructor de la entidad no recibe clave y la lista se rellena al cargar). Si el estilo no tiene hoja se usa el 2.
El monumento (clase 55) se resuelve como en `0x00521310`: con propietario válido, clase 55 con el índice del registro; sin él, la estatua de la ciudad (clase 10, clave `0x00672D28[id de ciudad]`, ya que `0x006DE4B8` es el array de ids de ciudad). Con eso no queda ningún objeto sin resolver en el guardado de prueba.

## Comprobación local

La [CLI de C10](C10_CLI.md) permite avanzar sin interfaz, guardar PAT y PST
nuevos y conservar el estado externo necesario para reanudar la ejecución.

```powershell
. ./scripts/Enter-P3Environment.ps1
$env:P3_SAVES = 'D:/AI/Patrician3/games'
$env:P3_CPR = 'D:/AI/Patrician3/games/iso-english/app/p2arch0_eng.cpr'
cargo test --manifest-path reimpl/Cargo.toml -p p3-mvp -p p3-render
```

Las pruebas con partidas y CPR reales se omiten si las variables no están
definidas. La prueba de render completo de las ciudades tarda más. El
contraste previo de proyección y de componentes económicos con x86 se
documenta en sus paquetes; el cliente integrado no se ha contrastado con una
captura ni con una sesión completa del juego.
