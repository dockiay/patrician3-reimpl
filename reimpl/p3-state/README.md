# p3-state: estado canónico, primera versión (S01)

`Snapshot::extract(&EconomyWorld)` produce un estado con **identificadores de dominio** (`TownId`, `MerchantId`, `OfficeId`, `FacilityId`, `ShipId`) en mapas ordenados; no usa direcciones de memoria ni posiciones de listas como identidad. Las cadenas del original (`Town+0x784` → oficinas → negocios, fin `0xFFFF`) pasan a listas ordenadas de identificadores; un ciclo, una referencia colgante o un elemento en dos cadenas devuelve `StateError` (no se corrige en silencio). `to_json()` da un texto canónico y `diff()` lista `ruta: antes → después`.

Subconjunto (primeros escenarios de compraventa y paso del tiempo): reloj (tick, mes, día del año); ciudad (existencias de 24 mercancías, umbrales de precio, población por grupo, dinero, banderas, instalaciones, oficinas); comerciante (oro, tipo, ciudad natal, ingresos); oficina (ciudad, armas, negocios); negocio (empleados, ciudad); barco (dueño, estado).

Pruebas (`cargo test -p p3-state`, 6): cadenas → listas, identidad estable al añadir elementos, texto determinista con ida y vuelta, diff con rutas, referencias rotas, y extracción sobre más de cien mundos de los casos de paridad x86 de C04 (comerciantes, barcos y reloj reales; las cadenas del fixture son aleatorias, por lo que sus ciudades son marcadores sin enlazar).

`towns_from_save(&Save)` (S02): las 24 ciudades de un `.pat` al mismo estado canónico, para comparar guardados entre sí (`tests/from_save.rs`, con `P3_SAVES`). Sin oficinas ni cadenas, que dependen de bloques aún no leídos.

## Límites

- Origen: el modelo `EconomyWorld` con paridad x86 y, solo para ciudades, los guardados; comerciantes, oficinas, negocios y barcos desde un `.pat` exigen leer sus bloques (S02/S04).
- Fuera: capitanes, convoyes, edificios (`buildings::Site`), eventos, misiones, mensajes y RNG; el barco solo trae dueño y estado, y la oficina no conoce a su comerciante en el modelo.
- No se compara con un estado extraído del original: falta B06 y los guardados de escenario de B05.
