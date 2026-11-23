# Lecturas de guion que bloqueaban el corpus C10

PC de Sergio, 4/11/2026. El corpus nativo de 30 días de `TEST25_ConvoyMar.pat`
y `ConvoyMar.pat` alcanzaba `WareVerstecken.p2m` (guion 3), primero el comando
`0x44` en el índice 12 y después `0x11` en el índice 15, tick 125748836.

- `0x44`, bloque `0x004F15AC` del intérprete `0x004ECF64`: cuenta la cadena
  que comienza en `merchant+0xC`, enlazada por `office+0x2C8`. Elige con resto
  **sin signo** `reloj % longitud` y devuelve el **byte** de ciudad
  `office+0x2C6`. No usa el enlace urbano `office+0x2CA` ni el RNG CRT.
- `0x11`, bloque `0x004EF112`: obtiene el comerciante por `0x005303C0` y
  copia su byte `+0x1D`, ampliado sin signo, a la variable destino. Se conserva
  el nombre por desplazamiento: esta conexión no demuestra su significado.
- `OfficeTownHost` prepara las imágenes actuales de oficinas para tareas
  `0x1B`, como ya hacía para otras tareas que las consumen. La ejecución
  prepara variables y tarea en copias antes de publicar el resultado.

`x86_c10_script_office_town.py` ejecuta 512 casos por comando dentro del
intérprete original, con accesores originales de comerciante y oficina.
Cubre cadenas permutadas, ranuras fuera de la cadena, centinelas, ticks
0/1/0x7fffffff/0x80000000/0xffffffff, los 256 valores de `merchant+0x1D` y
solapamiento entre variable de entrada y destino. Solo se interceptan las
fronteras comunes de limpieza del intérprete. Se comparan variables, PC y fecha.

La prueba integrada usa las imágenes de oficinas y comerciantes de la partida
local: seis propietarios, dos despachos cada uno y cadena `0x44→0x11→0xFD`.
Compara el planificador, intérprete y serializador originales con S05 y comprueba
dos guardados consecutivos y la recarga de las variables. Sus datos comerciales
no se versionan; el guion de prueba es sintético.

Reproducción con el entorno local configurado:

```powershell
cargo test -p p3-core --test c10_script_office_town_x86 --test c10_mission_basic_x86
cargo test -p p3-sim --test c10_script_office_town_live_x86 --test c10_mission_live_x86 --test c10_mission_ship_health_live_x86
```

Variables de entorno: `P3_EXE`, `P3_ORACLE_PY`, `P3_SAVES`, `P3_CPR`. Ejecutable
inglés SHA-256 `fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece`.

Límites: una cadena vacía provoca división por cero en el original y una cadena
cíclica puede no terminar; Rust las rechaza sin publicar el estado parcial. No
se han portado todos los comandos del intérprete. Esta evidencia aislada no
sustituye el corpus completo de 30 días ni implica validación en juego.
