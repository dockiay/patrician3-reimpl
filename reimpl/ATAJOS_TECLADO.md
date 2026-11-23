# Atajos de teclado del Patrician III original

Sacados del ejecutable inglés (`Patrician3.exe`) leyendo quién consulta la tabla de teclas `0x006DCBC0 + tecla` y el manejador
`game_screen_on_key_down` (`0x004247A0`). Estado en el cliente `p3-juego` comprobado el 17/11/2026 con la partida nueva del
enlazador (`cargo test --release -p p3-mvp --test ui_hotkeys`, necesita `P3_EXE` y `P3_CPR`).

## En la partida (ciudad o carta)

| Tecla | Qué hace | Dónde | En p3-juego |
|---|---|---|---|
| F5 | Alterna el minimapa entre la zona cercana y toda Europa | `game_screen_update` `0x0042D5EB` | Funciona |
| F6 | Abre la lista de barcos («Ship overview») | `game_screen_update_hud_and_hotkeys` `0x00424472` | Funciona |
| F7 | Abre las cartas personales | `0x004244B2` | Funciona |
| F8 | Abre la carta náutica | `0x0042D627` | Funciona |
| F12 | Avance rápido hasta el siguiente suceso (operación `0xC8`, modo 1); Esc, Intro o F12 lo cortan | `0x0042452E`, `game_screen_host_update` `0x00401B33` | Funciona (prueba `f12_fast_forward_runs_until_event`) |
| Pausa | Pausa y reanuda (operación `0xC9` 4) | `0x00424CAD` | Funciona |
| + / − del teclado numérico | Más rápido / más lento (`0xC9` 0 y 1). No en partida por turnos | `0x00424B9B` | Funciona |
| Tab | Siguiente barco propio en el puerto o en el mar, o siguiente ciudad con oficina; con Mayús (o Bloq Mayús) al revés | `0x00424BB6` | Funciona |
| Ctrl+1…9, 0 | Guarda los barcos seleccionados como grupo | `0x00424817` → `merchant_set_ship_group` | Sin probar con barcos elegidos |
| 1…9, 0 | Selecciona ese grupo de barcos | `game_screen_select_ship_group` `0x0042D750` | Sin probar con barcos elegidos |
| Intro | Abre o envía el chat (multijugador) | `0x004248D4` | Sin probar (multijugador) |
| Esc | Cierra la ventana emergente abierta; si no hay ninguna, abre Opciones | `0x004247DA` | Funciona (`ui_esc_burst`) |
| Ctrl+S | Captura de pantalla en la carpeta `grabs` | `present_frame` `0x004B9029` | **Arreglado 17/11**: antes no guardaba nada; ahora `grabs/grabNNNN.bmp` |
| Flechas | Desplazan la ciudad y la carta | `town_view_update_frame`, `nav_map_update` | Llegan al juego; sin prueba propia |
| Supr | Borra la carta seleccionada | `personal_letters_side_menu_update` | Sin probar |
| Ctrl + clic | Añadir barcos a la selección / ruta de varios barcos | `ship_list_window_on_mouse_event`, `scroll_map_commit_waypoint_route_for_selected_ships` | Sin probar |

## En la batalla naval (`SeaBattle::update_interface` `0x0060ED39`)

Espacio o 0 del teclado numérico: seleccionar todos los barcos propios. Re Pág / Av Pág: subir y bajar el nivel pedido de los
barcos elegidos. Fin: retirada. Inicio, Insert y Supr: órdenes y opciones del combate (ver `research/systems/barcos/COMBATE.md`).
Sin probar en el cliente.

## Ventanas y menús

Esc cierra o vuelve atrás en casi todas las pantallas (menú, carga, partida nueva, multijugador, cuadros de mensaje). Intro y
Espacio pasan los créditos y la pantalla final. En los cuadros de texto y números: Intro, Inicio, Fin, flechas, Retroceso y Supr.

El juego no tiene F1…F4, F9…F11 ni atajos de letras para los edificios.
