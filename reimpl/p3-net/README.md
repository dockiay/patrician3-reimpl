# p3-net: red del original (M03) y partida entre clientes nuevos (M02)

Nube Claude, 10/11/2026. Base: la especificación estática
[PRE-SPEC-NET](../../research/systems/borde-motor/net-protocol/README.md). Sin juego ni DLL
cargada; la firma del marco de `SchnellTransfer.dll` sigue sin conocerse (NET-Q1).

## M03: protocolo del original, con paridad x86

Catorce funciones del EXE inglés portadas y contrastadas en emulación (filas `M03` de
[`functions.tsv`](../functions.tsv)):

| Módulo | Qué cubre | Oráculo y casos |
|---|---|---|
| `wire` | Registro de 20 bytes, paquete de 16 + 52 registros (`0x420`), encadenado al llenar, marco de 8 bytes | `oracle/x86_net_packets.py`: 5.919 pasos (785 con encadenado) |
| `queue` | Cola de anillo de trozos de 0x100 registros (`Q_in`, `Q_out`), crecimiento y liberación | mismo oráculo, 23 pasos con crecimiento |
| `transfer` | Envío de partida (`0xFFFE`), estadísticas (`0xFFFF`) y ajustes (`0xFFFD`) en trozos de 0x400 con 4 créditos y acuse `0xC5` | `oracle/x86_net_session.py`: 483 trozos enviados, 1.253 paquetes recibidos |
| `session` | Pasada del anfitrión por las ranuras 1..7: apretón de manos (versión `0x85`, contraseña en `0xCE`), rechazo `0xD0`+`0xC7`, cola de salida | mismo oráculo: 150 pasadas, 112 rechazos |
| `sync` | Respuestas a `0xB8` (`0xB9`..`0xBE`) y suma `0xD4`; comparación del anfitrión (guardar «NETZWERK» y `0xBD`) | `oracle/x86_net_sync.py`: 300 respuestas (2.688 registros), 100 secuencias, 57 alarmas |

Hallazgos que no estaban en la especificación: el anfitrión usa **un único búfer de pila**
para todas las ranuras (un mensaje corto conserva los bytes de la ranura anterior); el
último trozo de estadísticas y los ajustes reponen un crédito antes de enviarse; tras las
estadísticas el emisor pasa a estable (3): los ajustes solo se envían en el estado 6,
después de un mapa personalizado.

`transport::TcpLink` envía y recibe con el marco de la DLL. `tests/tcp_session.rs` monta
una sesión TCP local entre anfitrión y cliente nuevos: contraseña aceptada, transferencia
de una partida real (194.172 bytes en 195 trozos, 49 acuses) con estadísticas y ajustes
idénticos al llegar, y contraseña errónea rechazada con `0xD0 {ranura, 2}` + `0xC7`.

Límites: la firma del marco es de ensayo (`P3T`); no se ha hablado con un original ni con
la DLL; la búsqueda LAN, el mapa personalizado (`0x0054BF80`) y la ejecución completa de
`execute_operations` siguen fuera. Un cliente nuevo frente a un anfitrión original necesita
además ejecutar las operaciones del juego igual que el original (eso es la simulación).

## M02: partida entre clientes nuevos

`lockstep` (protocolo propio, no el del original) conserva su modelo: el anfitrión es el
único secuenciador, difunde una pasada con las órdenes de los clientes por ranura, las
suyas, los avisos y el avance `0xC4 {tick, tick+8}`; todas las máquinas ejecutan solo lo
que llega en las pasadas. Añade lo que el original no tiene: unión y vuelta a mitad de
partida con instantánea y resincronización de todos, hash completo del estado en cada
pasada (más estricto que `0xD4`), ventana de adelanto, tiempo de espera con aviso de baja
(`0x1D3`, como `0xD3`) y reintento si la instantánea no se puede tomar (batalla en pausa).

`p3-mvp/src/net.rs` (`NetGame`) conecta la sesión del cliente: órdenes `0x1001` comercio
de oficina, `0x1002` comercio de barco, `0x1003` construir y `0x1004` zarpar, cada una
desde la oficina indicada; instantánea = `Session::save` más CRT, fiestas, marca de
noticias y oficina local.

Pruebas:
- `cargo test -p p3-net --test lockstep`: juego sintético; 4.000 órdenes de cuatro
  máquinas con 20–140 ms de latencia sin divergencias, ventana, baja y vuelta, divergencia
  local detectada en la pasada siguiente y versión rechazada.
- `cargo run --release -p p3-mvp --example m02 -- PARTIDA.pat p2arch0_eng.cpr --exe Patrician3.exe --days 30`:
  anfitrión y dos clientes con 30 y 120 ms abren la partida real, dan órdenes de comercio
  aleatorias, el segundo cliente se corta a un tercio y vuelve a la mitad. ConvoyHecho y
  TestAstiller: 980 pasadas (30 días), 1.790 órdenes, 1.769 hashes comparados y **0
  distintos**; baja anunciada, vuelta con tres resincronizaciones, sin pasadas aplazadas.
  Una unidad añadida a mano en un cliente se detecta en la pasada siguiente. Instantánea
  de unos 190 KB; reabrir cuesta unos 370 ms. Unos 48 s por partida.

Límites:
- Con `--humans 1,2` (comerciantes de IA convertidos en humanos con oficina propia) las
  máquinas siguen iguales (0 distintos en 1.274 hashes), pero el primer convoy humano
  nuevo llega a un efecto sin conectar (`navegación: efecto de convoy no conectado:
  msg_total`) y la batalla queda en pausa: las órdenes se rechazan y no se puede tomar la
  instantánea, así que el cliente cortado reintenta sin poder volver. Depende de C10
  (encuentros y efectos de convoy, de Codex).
- La sesión restaurada no es idéntica a la viva (planificador, matriz de IA y avisos se
  reconstruyen al abrir); por eso todos restauran a la vez al unirse alguien.
- Sin interfaz: la frontera son las órdenes de `Session`; no hay chat, lista de partidas
  ni migración de anfitrión. Nada de esto se ha probado en partida.

## WEB-7: multijugador por WebSocket con repetidor

Un navegador no abre TCP ni escucha puertos, así que el lockstep (`lockstep::Port`) tiene un
segundo transporte. El TCP del original (`transport::TcpLink`) no cambia.

| Pieza | Dónde | Qué hace |
|---|---|---|
| `relay::Relay` | `src/relay.rs` (sin E/S, compila a WASM) | Salas y reenvío en estrella: el anfitrión (máquina 0) habla con todos, cada cliente solo con él. No ejecuta el juego ni lee los mensajes. Sala = nombre; nace con el Hello del anfitrión y muere con él; una máquina caída libera su número y al volver ocupa el menor libre |
| `relay::RelayPort<L: Duplex>` | `src/relay.rs` | `Port` del lockstep sobre cualquier enlace de mensajes binarios. `p3-web` implementará `Duplex` con el `WebSocket` de JavaScript |
| `ws::WsLink`, `ws::Server` | `src/ws.rs`, característica `ws` (no wasm) | Cliente y servidor WebSocket nativos (tungstenite), un hilo por conexión |
| `p3-relay` | `src/bin/p3-relay.rs` | Servidor: `cargo run -p p3-net --features ws --bin p3-relay -- 0.0.0.0:8787` |
| `lockstep::cross_check` | `src/lockstep.rs` | Compara los hashes de todas las máquinas entre sí (no solo cliente contra anfitrión) |

Formato de los mensajes WebSocket (binarios): cliente→repetidor `1 Hello{host,sala}`,
`2 Data{to,bytes}`, `3 Bye`; repetidor→cliente `0x81 Assigned{peer}`, `0x82 Error{código}`,
`0x83 Data{from,bytes}`, `0x84 PeerGone{peer}`. Detalle en la cabecera de `relay.rs`.

Pruebas (`tests/relay.rs`, `cargo test -p p3-net --features ws`): salas, sala llena, errores,
expulsión de cliente a cliente; tres máquinas con juego sintético sobre un repetidor real
(instantáneas de 3 MB, 0 hashes distintos); caída y vuelta con el mismo número; muerte de sala.

Criterio de WEB-7 con el juego real, sin pantalla y sin cambiar el lockstep:

```
cargo build --release -p p3-mvp --example m02 --features p3-mvp/ws
target/release/examples/m02 AUTO.pat p2arch0_eng.cpr --days 30 --clients 2 --relay
```

Resultado (nube, 14/11/2026, `AUTO.pat` de `research/native-ui/2026-11-12/source-save`): 30 días,
anfitrión + 2 clientes por WebSocket, 980 pasadas, 1.781 hashes del estado completo comparados
(`cross_compared`) con **0 distintos**, 6.259 mensajes (1,2 MB) por el repetidor; un cliente cortado
a 1/3 y readmitido a 1/2 con resincronización; la alteración fuera del protocolo se detecta.
Con `--clients 1`: 799 comparaciones, 0 distintos. Con la red simulada (sin `--relay`) sigue igual:
1.769, 0 distintos. `cargo check -p p3-net --lib --target wasm32-unknown-unknown` pasa.

Límites: solo web↔web/escritorio con el lockstep propio (no habla con un original; NET-Q1 sigue
abierto). `--humans` (convoyes de humanos) sigue esperando a C10. El repetidor no autentica ni
cifra (usar `wss://` detrás de un proxy TLS); sin límite de tamaño de sala más allá de 8 máquinas.
El `Duplex` del navegador y el cliente de salas en `p3-web` quedan para ese hilo.
