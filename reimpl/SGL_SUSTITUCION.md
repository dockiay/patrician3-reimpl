# Capa sgl: qué se transcribe y qué se sustituye (11/11/2026)

Lote SGL de `coordination/DESGLOSE_PENDIENTES.md` (108 funciones).

- **Transcritas con el elevador** (`tools/lift.py` → `p3_core::lifted::sgl`, trazas x86 en `tests/fixtures/lifted_sgl_*.json`, prueba
  `x86_lifted.rs#lifted_sgl_match_x86`): 107 de 108, incluidos los 46 envoltorios `jmp [slot]`. Estos solo leen la ranura y llaman al
  anfitrión, así que el porte es fiel pero no hace nada por sí mismo.
- **Sustituidas en p3-render**: la tabla `p3_render::sgl_slots::SLOTS` asocia cada envoltorio con la rutina que lo cubre; desde el 11/11
  las 46 ranuras tienen sustitución (`rust: Some`, comprobado en `sgl_slots::tests`). Cada grupo se prueba contra `ddraw_Dll.dll` en Unicorn:
  - Dibujo: Blt/Blt_RC/StretchBitmapRect (`sgl::draw_*_rect`, J06), texto y StrLen (`sgl_font`), línea (`sgl::draw_line`, `0x4BB390`).
  - Recortes (`sgl_clip`, oráculo `x86_sgl_clip.py`): árbol de rectángulos del objeto `0x05050F64` nodo a nodo, modo 2D, rectángulo de
    recorte y recorrido de pasadas; 500 secuencias aleatorias (con divisiones de nodos) sin diferencias.
  - Texturas de memoria (`sgl_tex`, `x86_sgl_tex.py`): crear, bloquear/desbloquear, indicadores, paleta, clave y precarga; 400 secuencias.
  - Carga de `.aim` (`sgl_tex::convert_raw` y `TexStore::load/reload_memory_texture`, `x86_sgl_load.py` con `AIM.dll` + `ddraw_Dll.dll`):
    914 ficheros reales del archivo inglés (todos los de 24 y 32 bits y de bloques, uno de cada seis con paleta) y 233 recargas, mismos
    píxeles, paleta, indicadores y clave. Los bloques 0xF son de alfa explícito con colores ampliados por desplazamiento y tabla `n/3`;
    la recarga conserva la paleta anterior si la había. Los 5 `.aim` JPEG (`ijl11.dll`) y los formatos 0xD/0x11 (no hay en el corpus) quedan
    fuera. La prueba necesita `P3_CPR`.
  - Pantalla/Direct3D (`sgl_screen`, `x86_sgl_screen.py`): el dispositivo DirectDraw es de la plataforma (rasgo `ScreenDevice`); se reproduce
    la lógica de la DLL a su alrededor con un dispositivo simulado que anota cada llamada: `InitScreen`, `SwitchScreenMode`, `SwapBuffer`,
    `CleanUp`, `GrabScreen`, `MessageBox`, `SetArchive`, `GetProjectionData` (x87 en precisión simple y extendida), `DrawScene` sin mundo,
    `Begin3D`/`Finish3D` e `InitHardware`. 400 secuencias, 3.798 operaciones, 8.543 llamadas y 255 fallos de la DLL iguales.
  - Hallazgos: `[0x05080B40]` (pantalla principal) no se escribe nunca, así que `CleanUp` o un `InitScreen` fallido con pantalla activa leen
    un puntero nulo en la DLL original; `DrawScene` siempre devuelve 0 porque el EXE no crea mundo 3D; `AIM.dll` necesita su arranque
    (`0x10009B40`) para descodificar bloques.
- **Pendiente**: `0x4BC010 load_gl_cfg_and_init_graphics` (1.647 bytes, SEH, lee ficheros): el oráculo no produce casos; sin fila.

# Lista de dibujo (lote C22, 170 funciones)

Transcritas con el mismo elevador en `p3_core::lifted::c22` (prueba `x86_lifted.rs#lifted_c22_match_x86`): 162 con paridad x86 (161 filas nuevas
en `functions.tsv`, tarea C22). Sin fila (8):
- `text_layout_base_add_segment`, `class22_reset`, `class22_set_size`: el oráculo no obtiene casos válidos (0 trazas).
- `class22_apply_display_options`, `class22_draw_draw_list_to_scene_buffer`: llamada de más al retorno STOP tras una llamada indirecta
  (fallo conocido del elevador, ver nota del hilo de paneles).
- `scrolling_scene_buffer_add_dirty_rect_wrapped` / `_blit_wrapped`: recursión directa interceptada por el oráculo; `line_angle_and_length`: pila x87
  vacía en el original. Quedan anotadas para el hilo que amplía el elevador.
Regenerar: `python reimpl/tools/lift.py ...` y `oracle/lifttrace.py --shard K/4` (≈1 h con 4 procesos).
