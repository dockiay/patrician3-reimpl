"""Regenerate the selected-ship runtime routines with the current x86 stack/x87 lifter.

From reimpl: python tools/ui_rt_ship.py --binary PATH/Patrician3.exe
This only reads the commercial executable. It does not run or modify the game.
"""
import argparse
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as folder:
        source = Path(folder) / 'ship.rs'
        spec = Path(folder) / 'ship.json'
        env = dict(os.environ, PYTHONUTF8='1')
        subprocess.run([sys.executable, str(ROOT/'tools/lift.py'), '--binary', args.binary,
                        '--catalog', str(ROOT.parent/'research/catalog/functions.tsv'),
                        '--out', str(source), '--spec', str(spec), '--lax', '--ext', '--fpu-exit',
                        '0x48ee90', '0x4916e0'], env=env, check=True)
        text = source.read_text(encoding='utf-8').split('\npub fn call_by_name')[0]
    text = text.replace('use crate::economy::', 'use p3_core::economy::')
    text = text.replace('use crate::flatmem::', 'use p3_core::flatmem::')
    text = text.replace('use crate::x86rt::', 'use p3_core::x86rt::')
    text += '\npub static SHIP: [(u32, super::core_table::CoreFn); 2] = [(0x48ee90,ship_side_menu_draw_ship_info),(0x4916e0,ship_side_menu_draw_ship_row_owned)];\n'
    target = ROOT/'p3-mvp/src/ui_rt/ship_lifted.rs'
    target.write_text('\n'.join(line for line in text.splitlines() if line.strip())+'\n', encoding='utf-8', newline='\n')
    print(target)


if __name__ == '__main__':
    main()
