"""Generate secondary-activity handlers and their missing direct dependencies.

Reads the local English executable. Existing runtime adapters take precedence;
no original executable or save is changed. Keep stack/FPU behavior in lift.py.
"""
import argparse
import csv
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

from capstone.x86 import X86_OP_IMM
from lift import Lifter

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
CATALOG = PROJECT / 'research/catalog/functions.tsv'
OUT = ROOT / 'p3-mvp/src/ui_rt/activities_lifted.rs'
SPEC = ROOT / 'oracle/ui_activities_lifted.json'
# The catalog splits the mission interpreter into command labels and the
# convoy trading routine into an entry and body. These bounds end at the
# next independent routine, not at the catalog's first fragment.
SIZES = {0x4ECF64: 0x6300, 0x52BFE0: 0x6D7}
# These tables were added while the activity work was in progress. Their
# handlers are registered, but registration alone does not close their calls.
DEPENDENCY_MODULES = {'script_lifted.rs', 'church_lifted.rs', 'ayto_lifted.rs',
                      'hanse_lifted.rs', 'lender_lifted.rs', 'barn_lifted.rs',
                      'office_lifted.rs', 'tavern_lifted.rs'}


def table_addresses(text):
    return {int(a.replace('_', ''), 16) for a in re.findall(
        r'^\s*\(\s*(0x[0-9a-fA-F_]+),\s*[\w:]+', text, re.M)}


def existing():
    result = set()
    folders = ['p3-mvp/src/ui_rt', 'p3-render/src/j07', 'p3-render/src/j08',
               'p3-render/src/panels']
    for folder in folders:
        for path in (ROOT / folder).glob('*.rs'):
            if path == OUT:
                continue
            text = path.read_text(encoding='utf-8')
            result.update(table_addresses(text))
            if path.name in ('mod.rs', 'menus.rs', 'game_window.rs', 'win32.rs',
                             'display.rs', 'video_options.rs', 'zoom.rs') or 'p3-render' in str(path):
                result.update(int(a.replace('_', ''), 16) for a in re.findall(
                    r'^\s*(0x[0-9a-fA-F_]+)\s*=>', text, re.M))
    # Keep one authoritative full CFG translation for the mission interpreter;
    # script_lifted's historical entry has a shorter linear catalog extent.
    result.discard(0x4ECF64)
    return result


def collect_missing(lifter, seeds, registered, follow_registered):
    pending, visited, chosen = list(seeds), set(), set()
    while pending:
        address = pending.pop()
        if address in visited or address >= 0x639000:
            continue
        visited.add(address)
        if address in registered and address not in seeds | follow_registered:
            continue
        if address not in lifter.cat:
            raise SystemExit(f'Uncatalogued activity dependency {address:#x}')
        if address not in registered:
            chosen.add(address)
        extent = lifter.extent(address)
        instructions = (lifter.reachable_instructions(address, extent) if address == 0x4ECF64
                        else lifter.cs.disasm(lifter.code(address, extent), address))
        for ins in instructions:
            if ins.mnemonic in ('call', 'jmp') and ins.operands[0].type == X86_OP_IMM:
                target = ins.operands[0].imm
                if target in lifter.cat and target < 0x639000 and not address <= target < address + extent:
                    pending.append(target)
    return chosen


def activity_seeds():
    seeds = set()
    for line in (PROJECT / 'research/systems/operaciones/README.md').read_text(encoding='utf-8').splitlines():
        m = re.match(r'\|0x([0-9A-Fa-f]+)\|llamada\|`(0x[0-9A-Fa-f]+)`', line)
        if m and 0 <= int(m[1], 16) <= 0xC0:
            seeds.add(int(m[2], 16))
    tasks = (PROJECT / 'research/systems/ciudades/README.md').read_text(encoding='utf-8')
    seeds.update(int(a, 16) for a in re.findall(r'\|0x[0-9A-Fa-f]+\|`(0x[0-9A-Fa-f]+)`', tasks))
    seeds.update([0x4D7030, 0x4D74C0, 0x51AC60, 0x4DC6B0, 0x4DCD90, 0x53FA80,
                  0x443110, 0x4D6C90, 0x4D7B10, 0x526B40, 0x4869A0,
                  0x530E80, 0x535400, 0x5483D0,
                  0x4DD530, 0x4DD790, 0x4F3610, 0x4F3630, 0x4F3640, 0x4F3650,
                  0x4F69B0, 0x500F50, 0x5038F0, 0x505C90, 0x50FFB0, 0x5316E0,
                  0x442460, 0x4FBBA0, 0x5259D0, 0x50BC40, 0x50E340, 0x612930, 0x4D9D40,
                  0x442130,
                  # Long campaign/world progression reaches expeditions,
                  # defense works, fires and the battle update/placement path.
                  0x5029F0, 0x524CD0, 0x524F00, 0x5250D0, 0x534F10, 0x550550,
                  0x606CC0, 0x6079D0, 0x6197A0])
    return seeds


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--binary', required=True)
    ap.add_argument('--extra', action='append', default=[], help='Additional activity dependency, hex address')
    args = ap.parse_args()
    lifter = Lifter(args.binary, str(CATALOG))
    lifter.sizes.update(SIZES)
    registered = existing()
    seeds = activity_seeds() | {int(a, 16) for a in args.extra}
    follow_registered = set()
    modules = DEPENDENCY_MODULES | {p.name for p in OUT.parent.glob('grp_*_lifted.rs')}
    for name in modules:
        follow_registered.update(table_addresses((OUT.parent / name).read_text(encoding='utf-8')))
    chosen = collect_missing(lifter, seeds, registered, follow_registered)
    with tempfile.TemporaryDirectory() as tmp:
        src, spec = Path(tmp) / 'activities.rs', Path(tmp) / 'activities.json'
        cmd = [sys.executable, str(ROOT / 'tools/lift.py'), '--binary', args.binary,
               '--catalog', str(CATALOG), '--lax', '--ext', '--fpu-exit',
               '--out', str(src), '--spec', str(spec)]
        for address, size in SIZES.items():
            cmd += ['--size', f'{address:#x}={size:#x}']
        env = dict(os.environ, PYTHONUTF8='1')
        subprocess.run(cmd + [hex(a) for a in sorted(chosen - {0x4ECF64})], env=env, check=True)
        report = json.loads(spec.read_text(encoding='utf-8'))
        text = src.read_text(encoding='utf-8').split('\npub fn call_by_name')[0]
        if report['unsupported']:
            initial = report
            subprocess.run([v for v in cmd if v != '--ext'] + list(report['unsupported']), env=env, check=True)
            fallback = json.loads(spec.read_text(encoding='utf-8'))
            if fallback['unsupported']:
                raise SystemExit(fallback['unsupported'])
            text += '\npub fn ' + src.read_text(encoding='utf-8').split('\npub fn call_by_name')[0].split('pub fn ', 1)[1]
            report = dict(funcs=initial['funcs'] + fallback['funcs'], unsupported={})
        if 0x4ECF64 in chosen:
            subprocess.run(cmd + ['--reachable', '0x4ecf64'], env=env, check=True)
            mission = json.loads(spec.read_text(encoding='utf-8'))
            if mission['unsupported']:
                raise SystemExit(mission['unsupported'])
            text += '\npub fn ' + src.read_text(encoding='utf-8').split('\npub fn call_by_name')[0].split('pub fn ', 1)[1]
            report['funcs'] += mission['funcs']
    names = {f['name']: f['addr'] for f in report['funcs']}
    bodies = []
    for body in re.split(r'(?m)^(?=pub fn )', text)[1:]:
        name = re.match(r'pub fn (\w+)\(', body)[1]
        address = names[name]
        body = re.sub(r'^pub fn \w+\(', f'pub fn f_{address:08x}(', body, count=1)
        body = re.sub(r'c\.push\(m,(rm\(m,[^;]*?\))\);', r'{ let v = \1; c.push(m, v); }', body)
        # Helpers returning EDX:EAX or consuming x87 cannot use OpsHost's EAX-only return.
        for native in (0x63A6A0, 0x63A6E0, 0x63C610, 0x639C28):
            body = re.sub(rf'\{{ let args = \[[^\n]*\]; c.r\[0\] = host.call\(m, {native:#x}, c.r\[1\], &args\); \}}',
                          f'p3_core::economy::seh_pending::native(&mut c, m, {native:#x});', body)
        bodies.append(body)
    rows = sorted(f['addr'] for f in report['funcs'])
    header = ['//! Generated by `tools/ui_rt_activities.py`; regenerate instead of editing.',
              '#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]',
              'use p3_core::economy::ops_misc::OpsHost;', 'use p3_core::flatmem::Mem;',
              'use p3_core::x86rt::*;',
              f'pub static ACTIVITIES: [(u32, super::core_table::CoreFn); {len(rows)}] = [']
    header += [f'    ({a:#x}, f_{a:08x}),' for a in rows] + ['];', '']
    OUT.write_text('\n'.join(header) + ''.join(bodies), encoding='utf-8')
    report['seeds'] = sorted(seeds)
    report['limits'] = 'Generated code requires runtime path, ABI, effect and save/reload verification; registration is not acceptance.'
    SPEC.write_text(json.dumps(report, ensure_ascii=False, indent=1) + '\n', encoding='utf-8')
    print(f'{len(rows)} activity handlers/dependencies generated')


if __name__ == '__main__':
    main()
