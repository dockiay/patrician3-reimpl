"""Regenerate the two missing foreign-ship sidebar routines for the UI runtime."""
import argparse
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as directory:
        output = Path(directory) / "ship_panel.rs"
        subprocess.run([
            sys.executable, str(ROOT / "tools/lift.py"), "--binary", args.binary,
            "--catalog", str(ROOT.parent / "research/catalog/functions.tsv"),
            "--out", str(output), "--spec", str(Path(directory) / "spec.json"), "--lax", "--ext", "--fpu-exit",
            "0x00491240", "0x0051a3a0",
        ], check=True)
        source = output.read_text().replace("use crate::", "use p3_core::")
    source = source.replace(
        "pub fn ship_side_menu_draw_ship_row",
        "pub static SHIPS: [(u32, super::core_table::CoreFn); 2] = [\n"
        "    (0x00491240, ship_side_menu_draw_ship_row),\n"
        "    (0x0051a3a0, ship_get_index),\n];\n\n"
        "pub fn ship_side_menu_draw_ship_row", 1,
    )
    (ROOT / "p3-mvp/src/ui_rt/ship_panel_lifted.rs").write_text(source, encoding="utf-8")


if __name__ == "__main__":
    main()
