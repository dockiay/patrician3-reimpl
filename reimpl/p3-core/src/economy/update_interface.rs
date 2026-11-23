//! `update_interface` (`0x0060ED39`, thiscall sin argumentos): sincroniza la lista de objetos mostrados (`this+0x66C`, `+0xAD0` entradas) con el
//! vector devuelto por `0x4885A0`, avisa a las ventanas, actualiza el mapa de bits de avisos (`+0xACC`) y atiende las ocho marcas de la cola
//! de eventos de interfaz `0x6DCBE0..E4`, `0x6DCBEE`, `0x6DCC20` (cada una con el paso «desplazar y conservar», `v = (v>>1 & 0x7E) | (v & 0xFE)`).
//! El marco (`E-0x2C..E`) se modela con el `esp` de entrada: `E-0x28` temporal (`[esp+0x14]`), `E-0x29` pasada, `E-0x24` salida de `0x4885A0`,
//! `E-0x20/-0x1C/-0x18` contadores/guardas y desde `E-0x14` el registro de aviso. EAX queda sin valor definido (devuelve 0).

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

fn step(m: &mut dyn Mem, a: u32) -> bool {
    let v = m.r8(a);
    m.w8(a, ((v >> 1) & 0x7e) | (v & 0xfe));
    v != 0
}
fn r16(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r16(a))
}
fn r8(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r8(a))
}

pub fn update_interface(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let e = esp;
    let at = |o: u32| e.wrapping_sub(o);
    let (l14, l13, l18, l1c, l20, l24, rec) = (at(0x28), at(0x29), at(0x24), at(0x20), at(0x1c), at(0x18), at(0x14));
    // A: avisa por cada objeto con vector propio.
    if m.r32(this + 0xae0) != 0xffff_ffff && r16(m, this + 0xad0) != 0 {
        let mut ebx = 0u32;
        loop {
            let tbl = m.r32(this + 0x66c);
            let edi = ebx & 0xffff;
            let ship = m.r32(tbl.wrapping_add(edi * 4));
            let cl = r8(m, ship + 0x13d);
            let dx = r16(m, this + cl * 2 + 0x35a);
            if m.r32(0x6dfc14) == dx {
                let eax = m.r32(ship + 0x134);
                if eax != 0 && eax.wrapping_sub(4) != 0 {
                    let flag = u32::from(m.r8(this + 0x12e0) == 0);
                    host.call(m, 0x56dd20, 0, &[this + edi * 8 + 0xae0, this + 0x12d8, (eax & 0xffff_ff00) | flag]);
                }
            }
            if m.r8(this + 0x12e0) == 0 {
                m.w32(this + edi * 8 + 0xae0, 0xffff_ffff);
            }
            ebx = ebx.wrapping_add(1);
            if (ebx as u16) >= m.r16(this + 0xad0) {
                break;
            }
        }
    }
    // B: reconstruye la lista si el vector cambió.
    let t = m.r32(0x6ce6d0);
    let ebx = host.call(m, 0x4885a0, t, &[l18, 0, 1]);
    let edx = m.r32(l18);
    let cached = m.r32(this + 0x12e4);
    m.w32(l24, ebx);
    if edx != cached {
        let cl = r8(m, this + 0x558);
        let mut al = 0u32;
        m.w8(l14, 0);
        if cl != 0 {
            loop {
                let edx = m.r32(0x6dfc14);
                let edi = m.r32(l14) & 0xff;
                if r16(m, this + edi * 2 + 0x35a) == edx {
                    break;
                }
                al = (al + 1) & 0xff;
                m.w8(l14, al as u8);
                if al >= cl {
                    break;
                }
            }
        }
        let old = m.r32(this + 0x66c);
        host.call(m, 0x64f7e2, 0, &[old]);
        let edi = ebx & 0xffff;
        m.w32(l20, edi);
        let list = host.call(m, 0x64f7b9, 0, &[edi * 4]);
        m.w32(this + 0x66c, list);
        let mut ebp = 0u32;
        m.w8(l13, 0);
        loop {
            if (ebx as u16) != 0 {
                m.w32(l14, 0);
                m.w32(l1c, edi);
                loop {
                    let pass = r8(m, l13);
                    let head = if pass == 0 { r16(m, this + 0x10) } else { r16(m, this + 0x65c) };
                    if head != 0xffff {
                        let ecx = m.r32(l18);
                        let off = m.r32(l14);
                        let tab = m.r32(this + 4);
                        let want = r16(m, off.wrapping_add(ecx));
                        let mut cur = head;
                        loop {
                            let obj = m.r32(tab.wrapping_add(cur * 4));
                            if want == m.r32(obj.wrapping_add(4)) {
                                let lst = m.r32(this + 0x66c);
                                let o2 = m.r32(tab.wrapping_add(cur * 4));
                                m.w32(lst.wrapping_add((ebp & 0xffff) * 4), o2);
                                ebp = ebp.wrapping_add(1);
                                break;
                            }
                            let nx = m.r32(this + 0xc);
                            let nx = r16(m, nx.wrapping_add(cur * 2));
                            if nx == 0xffff {
                                break;
                            }
                            cur = nx;
                        }
                    }
                    let c = m.r32(l14).wrapping_add(2);
                    let n = m.r32(l1c).wrapping_sub(1);
                    m.w32(l14, c);
                    m.w32(l1c, n);
                    if n == 0 {
                        break;
                    }
                }
            }
            let p = r8(m, l13).wrapping_add(1) & 0xff;
            m.w8(l13, p as u8);
            if p >= 2 {
                break;
            }
        }
        let v = m.r32(l18);
        m.w16(this + 0xad0, ebp as u16);
        m.w32(this + 0x12e4, v);
    }
    // C: notifica a los objetos y a los avisos.
    let n1 = r16(m, this + 0x14);
    if n1 != 0 {
        let mut edi = 0u32;
        loop {
            let tab = m.r32(this + 4);
            let obj = m.r32(tab.wrapping_add((edi & 0xffff) * 4));
            host.call(m, 0x62018b, obj, &[]);
            edi += 1;
            if (edi as u16) >= m.r16(this + 0x14) {
                break;
            }
        }
    }
    if r16(m, this + 0xacc) != 0 {
        let mut edi = 0u32;
        loop {
            let tab = m.r32(this + 0x668);
            let p = m.r32(tab.wrapping_add((edi & 0xffff) * 4));
            if p != 0 {
                host.call(m, 0x6038c0, p, &[]);
            }
            edi += 1;
            if (edi as u16) >= m.r16(this + 0xacc) {
                break;
            }
        }
    }
    // D: mapa de bits de avisos.
    let a = step(m, 0x6dcbe0);
    let run_d = if a { true } else { step(m, 0x6dcc20) };
    if run_d {
        m.w8(0x6dcbe0, 0);
        m.w8(0x6dcc20, 0);
        let ax = r16(m, this + 0x14);
        let al = ((ax + 0x3f) >> 6) as u8;
        m.w8(l14, al);
        let edi = m.r32(l14) & 0xff;
        let p = host.call(m, 0x64f7b9, 0, &[edi * 8]);
        if r16(m, this + 0xad0) != 0 {
            for i in 0..(edi * 2) {
                m.w32(p.wrapping_add(i * 4), 0);
            }
            let mut edx = 0u32;
            loop {
                let tbl = m.r32(this + 0x66c);
                let ship = m.r32(tbl.wrapping_add((edx & 0xffff) * 4));
                let cl = r8(m, ship + 0x13d);
                let di = r16(m, this + cl * 2 + 0x35a);
                if m.r32(0x6dfc14) == di {
                    let cx = r16(m, ship + 0x138);
                    m.w8(l24, (cx >> 5) as u8);
                    let idx = m.r32(l24) & 0xff;
                    let q = p.wrapping_add(idx * 4);
                    let v = m.r32(q) | (1u32 << (cx & 0x1f));
                    m.w32(q, v);
                }
                edx += 1;
                if (edx as u16) >= m.r16(this + 0xad0) {
                    break;
                }
            }
            let cnt = r8(m, l14);
            if cnt != 0 {
                let mut bl = 0u32;
                loop {
                    let edi = (bl * 2) & 0xffff;
                    let lo = m.r32(p.wrapping_add(edi * 4));
                    let hi = m.r32(p.wrapping_add(edi * 4 + 4));
                    if lo != 0 || hi != 0 {
                        let cx = r16(m, 0x6df346);
                        let diff = 0x34u32.wrapping_sub(cx);
                        if diff > 0x1a {
                            host.call(m, 0x535740, rec, &[]);
                            let t = r8(m, this + 0x662);
                            m.w8(at(0x10), t as u8);
                            m.w32(at(0xc), hi);
                            m.w32(rec, 0x92);
                            m.w8(at(0xe), bl as u8);
                            m.w8(at(0xf), 1);
                            m.w32(at(8), lo);
                            host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
                        }
                    }
                    bl = (bl + 1) & 0xff;
                    if bl >= r8(m, l14) {
                        break;
                    }
                }
            }
        }
        host.call(m, 0x64f7e2, 0, &[p]);
    }
    // E: ¿algún objeto sin +0x13E? -> 0x6103E9(flag, 0)
    if step(m, 0x6dcbee) {
        m.w8(0x6dcbee, 0);
        let ax = r16(m, this + 0xad0);
        if ax != 0 {
            let mut cl = 0u32;
            m.w8(l14, 0);
            m.w8(l1c, 0);
            loop {
                let tbl = m.r32(this + 0x66c);
                let edi = m.r32(l1c) & 0xff;
                let ship = m.r32(tbl.wrapping_add(edi * 4));
                if m.r8(ship + 0x13e) == 0 {
                    m.w8(l14, 1);
                }
                cl = (cl + 1) & 0xff;
                m.w8(l1c, cl as u8);
                if cl >= ax {
                    break;
                }
            }
            let v = m.r32(l14);
            host.call(m, 0x6103e9, this, &[v, 0]);
        }
    }
    // F: ¿algún objeto sin la bandera 0x80 de +0x12A? -> 0x6103E9(flag, flag)
    if step(m, 0x6dcbed) {
        m.w8(0x6dcbed, 0);
        let ax = r16(m, this + 0xad0);
        if ax != 0 {
            let mut cl = 0u32;
            m.w8(l14, 0);
            m.w8(l1c, 0);
            let tbl = m.r32(this + 0x66c);
            loop {
                let edi = m.r32(l1c) & 0xff;
                let ship = m.r32(tbl.wrapping_add(edi * 4));
                if m.r8(ship + 0x12a) & 0x80 == 0 {
                    m.w8(l14, 1);
                }
                cl = (cl + 1) & 0xff;
                m.w8(l1c, cl as u8);
                if cl >= ax {
                    break;
                }
            }
            let v = m.r32(l14);
            host.call(m, 0x6103e9, this, &[v, v]);
        }
    }
    // G: ¿algún objeto sin la bandera 1 de +0x12A? -> 0x610FD9(flag)
    if step(m, 0x6dcbe4) {
        m.w8(0x6dcbe4, 0);
        let ax = r16(m, this + 0xad0);
        if ax != 0 {
            let mut cl = 0u32;
            m.w8(l14, 0);
            m.w8(l1c, 0);
            let tbl = m.r32(this + 0x66c);
            loop {
                let edi = m.r32(l1c) & 0xff;
                let ship = m.r32(tbl.wrapping_add(edi * 4));
                if m.r8(ship + 0x12a) & 1 == 0 {
                    m.w8(l14, 1);
                }
                cl = (cl + 1) & 0xff;
                m.w8(l1c, cl as u8);
                if cl >= ax {
                    break;
                }
            }
            let v = m.r32(l14);
            host.call(m, 0x610fd9, this, &[v]);
        }
    }
    // H: cambio de la ciudad actual (0x610D69).
    if step(m, 0x6dcbe3) {
        m.w8(0x6dcbe3, 0);
        let r = host.call(m, 0x610d69, this, &[]);
        if (r >> 8) as u8 & 0x80 != 0 {
            host.call(m, 0x535740, rec, &[]);
            let ecx = m.r32(0x6dfc14);
            let ax = r16(m, this + 0x662);
            m.w32(at(0xc), ecx);
            m.w32(rec, 0xa0);
            m.w32(at(0x10), ax);
            host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
        } else if r16(m, this + 0xad0) != 0 {
            let lst = m.r32(this + 0x66c);
            let ship0 = m.r32(lst);
            let vt = m.r32(this);
            let dl = m.r8(ship0 + 0x13d);
            m.w8(l24, dl);
            let f = m.r32(vt.wrapping_add(0x28));
            let r2 = host.call(m, f, this, &[]);
            if r2 as u8 == 0 {
                let cx = m.r32(l24) & 0xff;
                if m.r8(cx + this + 0x559) != 0 && m.r8(this + 0xacb) <= 3 {
                    let mut ax = r16(m, this + 0x10);
                    if ax != 0xffff {
                        let tab = m.r32(this + 4);
                        loop {
                            let obj = m.r32(tab.wrapping_add(ax * 4));
                            if m.r8(obj + 0x12a) & 0x66 == 0 {
                                host.call(m, 0x610c09, this, &[]);
                                break;
                            }
                            let nx = m.r32(this + 0xc);
                            let nx = r16(m, nx.wrapping_add(ax * 2));
                            if nx == 0xffff {
                                break;
                            }
                            ax = nx;
                        }
                    }
                }
            }
        }
    }
    // I: nivel máximo (+0x148) + 1 (máx. 4) -> 0x610579.
    if step(m, 0x6dcbe1) {
        m.w8(0x6dcbe1, 0);
        let ax = r16(m, this + 0xad0);
        if ax != 0 {
            let mut bl = 0u32;
            m.w8(l14, 0);
            let mut edx = m.r32(this + 0x66c);
            for _ in 0..ax {
                let ship = m.r32(edx);
                let cl = r8(m, ship + 0x148);
                if bl < cl {
                    bl = cl;
                }
                edx = edx.wrapping_add(4);
            }
            m.w8(l14, bl as u8);
            if bl < 4 {
                bl += 1;
                m.w8(l14, bl as u8);
            }
            let v = m.r32(l14);
            host.call(m, 0x610579, this, &[v]);
        }
    }
    // J: nivel mínimo (+0x148, tope 4) - 1 -> 0x610579.
    if step(m, 0x6dcbe2) {
        m.w8(0x6dcbe2, 0);
        let ax = r16(m, this + 0xad0);
        if ax != 0 {
            let mut bl = 4u32;
            m.w8(l14, 4);
            let mut edx = m.r32(this + 0x66c);
            for _ in 0..ax {
                let ship = m.r32(edx);
                let cl = r8(m, ship + 0x148);
                if bl > cl {
                    bl = cl;
                }
                edx = edx.wrapping_add(4);
            }
            m.w8(l14, bl as u8);
            if bl != 0 {
                bl -= 1;
                m.w8(l14, bl as u8);
            }
            let v = m.r32(l14);
            host.call(m, 0x610579, this, &[v]);
        }
    }
    let _ = l13;
    0
}
