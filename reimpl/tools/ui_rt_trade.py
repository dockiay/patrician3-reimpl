"""Regenerate the market transaction runtime routines with the current x86 stack/x87 lifter.

From reimpl: python tools/ui_rt_trade.py --binary PATH/Patrician3.exe
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
        source = Path(folder) / 'trade.rs'
        spec = Path(folder) / 'trade.json'
        env = dict(os.environ, PYTHONUTF8='1')
        subprocess.run([sys.executable, str(ROOT/'tools/lift.py'), '--binary', args.binary,
                        '--catalog', str(ROOT.parent/'research/catalog/functions.tsv'),
                        '--out', str(source), '--spec', str(spec), '--lax', '--ext', '--fpu-exit',
                        '0x537340', '0x537450', '0x53d2b0', '0x53d3c0', '0x52f3e0', '0x517fe0', '0x5308a0', '0x5007a0', '0x5006f0'], env=env, check=True)
        spec_text = spec.read_text(encoding='utf-8')
        text = source.read_text(encoding='utf-8').split('\npub fn call_by_name')[0]
    text = text.replace('use crate::economy::', 'use p3_core::economy::')
    text = text.replace('use crate::flatmem::', 'use p3_core::flatmem::')
    text = text.replace('use crate::x86rt::', 'use p3_core::x86rt::')
    import json
    if json.loads(spec_text)['unsupported']:
        raise SystemExit(json.loads(spec_text)['unsupported'])
    names = [(f['addr'], f['name']) for f in json.loads(spec_text)['funcs']]
    text += '\npub static TRADE: [(u32, super::core_table::CoreFn); %d] = [\n'%len(names)
    text += ''.join('(0x%x,%s),\n'%(addr,name) for addr,name in names)+'];\n'
    target = ROOT/'p3-mvp/src/ui_rt/trade_lifted.rs'
    target.write_text('\n'.join(line for line in text.splitlines() if line.strip())+'\n', encoding='utf-8', newline='\n')
    print(target)


if __name__ == '__main__':
    main()
