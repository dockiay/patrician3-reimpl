# Elevador ampliado: pila, SEH y x87 (11/11/2026)

`reimpl/tools/lift.py` traduce funciones del Patrician3.exe inglés a Rust sobre `p3_core::x86rt`. Lo que sigue es lo que añadió el hilo
«Ampliar el elevador para pila y x87» (lotes ELV-PILA y ELV-X87 de `coordination/DESGLOSE_PENDIENTES.md`). La interfaz no cambia
(`pub fn nombre(m, host, regs) -> [u32; 8]` y `call_by_name`), y regenerar los módulos de diálogos con `--lax` da el mismo Rust que antes.

## Cómo funciona

- **Dos intentos.** `lift.py` prueba primero el modo de siempre. Si falla por la pila (`pila ...`, `escritura de esp`, `x87 ...`), repite con el
  **modo ampliado** (`--ext` lo fuerza). Un fallo en el modo ampliado se devuelve como «sin soporte» con el motivo.
- **Estado de pila.** `plan_depth` recorre el grafo con el estado `(base, profundidad, ebp, guardas tardías)`. Entiende `mov ebp, esp`,
  `mov esp, ebp`, `lea esp, [ebp-N]`, `leave`, `and esp, -8` (nueva base) y las guardas de registros que se apilan después de otras instrucciones.
  Las confluencias deben coincidir y cada `ret` debe dejar profundidad 0.
- **Argumentos de las llamadas indirectas** (`call [edx+8]`, `call ebp`…). Se deducen como *profundidad actual − profundidad de marco*, menos lo que
  exigen después los consumidores conocidos del mismo tramo (llamadas directas con `ret N`, `add esp`). Hay dos hipótesis para las llamadas
  indirectas posteriores (retiran sus argumentos, o los dejan para un `add esp` común como las clases sgl cdecl) y vale la que deja la pila vacía.
  **Límite conservado:** igual que en el elevador de diálogos, la cuenta de argumentos de una llamada virtual sale del patrón de pila (equilibrio
  comprobado), no del método real.
- **Llamados.** En modo ampliado `callee_pops` recorre el flujo desde la entrada (no desensamblado lineal, que se pierde con tablas de saltos
  incrustadas) y los envoltorios `jmp [IAT]` stdcall usan los parámetros del catálogo.
- **Instrucciones nuevas.** `jp/jnp` (PF en `Cpu`), `rep stosd/stosb/movsb`, `stos`/`movs` sueltos, `cld`, `mov esp, reg`, `and esp, imm`, y
  x87: `fchs`, `fabs`, `fsqrt`, `fxch`, `fst/fstp st(i)`, `fadd/fmul/fsub/fdiv st(i)` y sus formas con `p`, `fcom/fcomp`, `fistp`, `fnstsw`.
  Pila x87 de 8 entradas con la semántica de anillo de QEMU/Unicorn (al desbordar se pierde el fondo; leer de vacío da 0.0).
- **Fragmentos `_tail`.** Los que empiezan a mitad de una función con marco `ebp` toman `ebp` como el `ebp` guardado.

## Trazas

- `--fpu-exit` hace que cada función con x87 deje la profundidad y ST0 de la pila x87 en `x86rt::FPU_EXIT`; `lifttrace.py` los lee del Unicorn
  (ejecuta un `fstp qword` en una página aparte) y `x86_lifted.rs` los compara. Hay ~14 funciones x87 no verificables (euler_*, rotation_matrix_*, arrow_outline_build, init_economy, siege_state_tick…): llaman a rutinas del CRT interceptadas que no reproducen su efecto en la FPU; quedan fuera de la lista de aprobadas.
- Lotes: `p3-core/src/lifted/elv1..4.rs`, `oracle/lifted_elv*.json`; fixtures `lifted_elv*_*.json`; prueba `lifted_elv_match_x86`
  (`tests/fixtures/lifted_elv_passed.txt` lista las que deben coincidir; `P3_LIFT_REPORT_ELV=ruta` la regenera).
- Memoria: seis procesos de `lifttrace.py` a la vez agotan 16 GB; usa como mucho cuatro.

## Lo que no traduce todavía

Continuaciones separadas por el catálogo (`main_menu_init` salta a `main_menu_create_screens_tail` con el marco vivo), funciones con datos
incrustados que el catálogo corta a mitad de instrucción (`load_save_screen_load_game_file`, …) y varias con pila no deducible (cleanups de cdecl
mezclados entre ramas). Ver `coordination/DESGLOSE_PENDIENTES.md`.
