# Reproducir las pruebas de `reimpl/` (B07)

Un solo comando, con las versiones y las exclusiones registradas:

```
python scripts/reimpl_check.py              # Rust: lo que no necesita datos siempre corre; el resto se excluye con motivo
python scripts/reimpl_check.py --oracles    # además los oráculos x86 con --check (necesita el EXE; lento)
python scripts/reimpl_check.py --no-data    # lo que ve la integración continua (.github/workflows/reimpl.yml)
python scripts/reimpl_check.py --strict     # falla si falta algún dato o su SHA-256 no coincide
```

El informe (`artifacts/reimpl-check/report.md` y `.json`, carpeta ignorada por Git) lleva: commit y si había cambios sin confirmar,
`rustc`/`cargo`, paquetes de Python, SHA-256 de `Cargo.lock` y de cada dato local, recuento por binario de pruebas, fallos, y la lista de
**exclusiones con su motivo**. Una exclusión por falta de datos no es un fallo, pero nunca es silenciosa: las pruebas con `#[ignore = "motivo"]`
se enumeran con su motivo y las que se saltan en tiempo de ejecución imprimen un aviso (`OMITIDA`, «sin definir», «omitida») que el
script recoge.

## Versiones fijadas

- Rust: `reimpl/rust-toolchain.toml` (1.97.0); dependencias: `reimpl/Cargo.lock` (`--locked`).
- Python de los oráculos: `config/reimpl-requirements.lock.txt` (unicorn, capstone, pefile); no hay más dependencias.

## Datos locales (no se versionan)

| variable | dato | SHA-256 | qué habilita |
|---|---|---|---|
| `P3_EXE` | `Patrician3.exe` inglés | `fe436dc5…eddece` | oráculos x86 (`--oracles`), regenerar capturas |
| `P3_CPR` | `p2arch0_eng.cpr` | `6ebd9aa5…079009` | p3-assets con ficheros reales (CPR, LET, AIM, `.town`, `.spawn`, `.nodes`, fuzzing) y navegación real |
| `P3_AIM_DLL` | `AIM.dll` | `4c75cb3e…1c72` | regenerar `aim_corpus.json` (`reimpl/oracle/aim`) |

Sin variable, el script busca `/home/claude/exe/`, y `…/Patrician3/games/iso-english/app/` (carpeta traída del PC con `device_stage_files`).

**Navegación (antes ausente):** `scripts/reimpl_check.py` extrae `navdata/nav_matrix.dat`, `nav_vec.dat` y `matrix_int.dat` del CPR y, con el
EXE, regenera `real_select.json` y `real_routes.json` con `reimpl/oracle/x86_planner_real.py` (variables `P3_NAVDATA`, `P3_EXE`); las dos
pruebas de datos reales de `x86_route_planner.rs` pasan a ejecutarse. Los corpus ampliados opcionales (`P3_SMOOTH_EXTRA`, `P3_SEA_EXTRA`) no se
regeneran: ninguna prueba obligatoria depende de ellos.

## Qué se excluye siempre fuera del PC de Sergio (27 pruebas de `p3-audio`)

Las 27 pruebas `#[ignore]` de audio necesitan los 123 MP3 comerciales (`P3_MP3_ROOT`), capturas x86 del SDK Miles y manifiestos locales
(`P3_*_FIXTURE`) que se generaron en el PC y están en carpetas ignoradas; no hay forma de regenerarlas en la nube sin esas DLL. Quedan enumeradas
en cada informe con su motivo y no cuentan como paridad hasta que alguien las ejecute allí. En el PC: definir las variables que cada motivo
indica y ejecutar `cargo test --release -p p3-audio -- --ignored` (usar `--release`).

## Estado verificado (27/10/2026, nube Claude)

Con datos: 256 pruebas correctas, 0 fallidas, 27 excluidas (todas de audio) en 107 binarios, incluidas las de navegación real y las de
`p3-assets` con el archivo inglés. Sin datos (`--no-data`): mismas 256 correctas y 34 avisos de exclusión (27 de audio, 5 de `p3-assets`
con `P3_CPR`, 2 de navegación). Los oráculos x86 no se ejecutaron en este informe (`--oracles` es opcional y lento).
