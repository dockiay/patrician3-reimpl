#!/usr/bin/env python3
"""Registra en reimpl/functions.tsv las funciones de los módulos `sobrantes` (`lifted::sob`, `sob2` y `x87fix`) que coinciden con el x86 en
`lifted_sob_match_x86` (hilo «Portar sobrantes de dibujo, ciudad y _ftol»).

Uso: P3_LIFT_REPORT_SOB=/tmp/rep.txt cargo test -p p3-core --test x86_lifted lifted_sob; python tools/sob_rows.py /tmp/rep.txt

El informe lleva una línea `nombre casos` por función correcta. Se escribe `p3-core/tests/fixtures/lifted_sob_passed.txt` (las que deben seguir
coincidiendo) con las que tienen al menos 3 casos. Si la dirección ya tenía fila (versión anterior sin `_ftol`/x87 en línea), se sustituye
su módulo, su prueba y su nota; si no, se añade una fila nueva."""
import csv, json, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RE = ROOT / "reimpl"
report = {}
for line in Path(sys.argv[1]).read_text().splitlines():
    if line.strip():
        n, c = line.split()
        report[n] = int(c)

mods = {"sob": RE / "oracle/lifted_sob.json", "sob2": RE / "oracle/lifted_sob2.json", "x87fix": RE / "oracle/lifted_x87fix.json"}
cat = {r["name"]: r for r in csv.DictReader(open(ROOT / "research/catalog/functions.tsv"), delimiter="\t")}
# tarea de cada lote (desglose de pendientes): se toma de la fila previa si existe; si no, de esta tabla por módulo de origen
LOT = {
    "c22": "C22", "walkers": "PORT-CIUDAD", "objects": "PORT-CIUDAD", "sgl": "SGL", "bas": "DESGLOSE-MAP-DBG-BAS", "map": "DESGLOSE-MAP-DBG-BAS",
    "dbg": "DESGLOSE-MAP-DBG-BAS", "dialogs": "J03", "dialogs2": "J03", "bld1": "BLD-1", "bld2": "BLD-2", "bld3": "BLD-3", "scr1": "SCR-1",
    "scr2": "SCR-2", "panels": "J03-PANELES", "panels2": "J03-PANELES", "panels3": "J03-PANELES", "panels4": "J03-PANELES", "siege_view": "SIE",
}
NEW_TASK = {n: "C22" for n in ("text_layout_base_add_segment class22_reset class22_resize_view class22_update_scene_buffer_cells_and_animations class22_set_size "
                                "class22_apply_display_options scrolling_scene_buffer_blit_wrapped scrolling_scene_buffer_add_dirty_rect_wrapped line_angle_and_length").split()}
NEW_TASK.update({n: "PORT-CIUDAD" for n in ("building_scene_window_draw town_view_ship_set_pose harbor_ship_sprite_get_name_label_rect map_object_table_rebuild_kind_index "
                                            "town_view_enter_battle_location town_view_switch_to_town town_view_free_path_tables town_view_unload_scene town_view_update_frame "
                                            "town_streak_line_invalidate_rect town_streak_line_is_on_screen").split()})
NEW_TASK.update({"load_gl_cfg_and_init_graphics": "SGL", "std_map_erase_subtree": "DESGLOSE-MAP-DBG-BAS", "convoy_choice_window_layout": "DESGLOSE-MAP-DBG-BAS",
                 "church_side_menu_on_show": "J03-PANELES", "game_screen_create_side_menus_and_dialogs": "J03", "motion2d_load_from_ini": "C22"})
tsv = RE / "functions.tsv"
lines = tsv.read_text().splitlines()
head, body = lines[0], lines[1:]
index = {}
for i, l in enumerate(body):
    index.setdefault(l.split("\t")[0].lower(), i)
passed = []
added = replaced = 0
TEST = "reimpl/p3-core/tests/x86_lifted.rs#lifted_sob_match_x86"
for mod, spec_path in mods.items():
    spec = json.load(open(spec_path))
    for f in spec["funcs"]:
        n = f["name"]
        if report.get(n, 0) < 3:
            continue
        passed.append(n)
        addr = "0x%08X" % f["addr"]
        old = body[index[addr.lower()]].split("\t") if addr.lower() in index else None
        task = "J08"
        if old:
            mm = old[2].split("::")
            if len(mm) >= 3 and mm[2] in LOT:
                task = LOT[mm[2]]
            elif old[3]:
                task = old[3]
        else:
            task = NEW_TASK.get(n, "J08")
        x87 = " _ftol/sqrt en línea (Cpu::crt_call), retornos x87 de los llamados (Cpu::fret) y pila x87 de salida comparada." if f.get("x87") else ""
        ind = " Llamadas virtuales: cuenta de argumentos deducida del patrón de pila, no contra el método real." if f.get("indirect") else ""
        note = (f"Portada automáticamente (reimpl/tools/lift.py --fpu-exit, modelo p3_core::x86rt), hilo «sobrantes»; {report[n]} trazas x86 "
                f"(oracle/lifttrace.py y/o oracle/snaptrace.py sobre volcados reales); registros y bytes escritos idénticos.{x87}{ind}")
        row = "\t".join([addr, "probada_emulador", f"p3_core::lifted::{mod}::{n}", task, TEST, note])
        if old:
            body[index[addr.lower()]] = row
            replaced += 1
        else:
            body.append(row)
            index[addr.lower()] = len(body) - 1
            added += 1
tsv.write_text("\n".join([head] + body) + "\n")
(RE / "p3-core/tests/fixtures/lifted_sob_passed.txt").write_text("\n".join(sorted(passed)) + "\n")
print(f"{added} filas añadidas, {replaced} sustituidas, {len(passed)} en lifted_sob_passed.txt")
