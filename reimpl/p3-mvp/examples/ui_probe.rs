//! Inventario dinámico de funciones sin enlazar: llama a cada dirección del catálogo en el enlazador `ui_rt` (ecx = 0, sin
//! argumentos, límite de llamadas pequeño) y anota si el enlazador la resuelve o la registra en `Runtime::missing`; de paso
//! recoge las demás ausentes que encuentra al ejecutar las resueltas.
//! Uso: `ui_probe Patrician3.exe p2arch0_eng.cpr direcciones.txt salida.tsv [INICIO]` (una dirección hex por línea). Si una
//! función se cuelga (bucle sin llamadas), el vigilante sale con código 3 tras anotar `colgada`; el guion que lo lanza reanuda en INICIO+1.
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let exe = std::fs::read(&a[0]).map_err(|e| e.to_string())?;
    let cpr = std::fs::read(&a[1]).map_err(|e| e.to_string())?;
    let addrs: Vec<u32> = std::fs::read_to_string(&a[2]).map_err(|e| e.to_string())?.lines().filter_map(|l| u32::from_str_radix(l.trim().trim_start_matches("0x"), 16).ok()).collect();
    let start: usize = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("{e:?}"))?;
    let mut mem = p3_mvp::ui_rt::PageMem::default();
    let mut rt = p3_mvp::ui_rt::Runtime::new(&mut mem, &exe, &archive, 800, 600)?;
    let mut out = std::fs::OpenOptions::new().create(true).append(true).open(&a[3]).map_err(|e| e.to_string())?;
    static CUR: AtomicU64 = AtomicU64::new(u64::MAX);
    static TICK: AtomicU64 = AtomicU64::new(0);
    let hang_out = a[3].clone();
    let addrs2 = addrs.clone();
    std::thread::spawn(move || {
        let mut last = (u64::MAX, 0u64);
        loop {
            std::thread::sleep(std::time::Duration::from_secs(4));
            let cur = (CUR.load(Ordering::SeqCst), TICK.load(Ordering::SeqCst));
            if cur == last && cur.0 != u64::MAX {
                let mut o = std::fs::OpenOptions::new().append(true).open(&hang_out).unwrap();
                writeln!(o, "{:#x}\tcolgada\t", addrs2[cur.0 as usize]).unwrap();
                std::process::exit(3);
            }
            last = cur;
        }
    });
    std::panic::set_hook(Box::new(|_| {}));
    for (i, &f) in addrs.iter().enumerate().skip(start) {
        CUR.store(i as u64, Ordering::SeqCst);
        TICK.fetch_add(1, Ordering::SeqCst);
        rt.missing.clear();
        rt.reset_probe();
        rt.call_limit = rt.calls + 400;
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| rt.call(&mut mem, f, 0, &[])));
        let panicked = r.is_err();
        let miss: Vec<u32> = rt.missing.keys().copied().collect();
        let status = if miss.contains(&f) { "ausente" } else if panicked { "resuelta(pánico)" } else { "resuelta" };
        let others: Vec<String> = miss.iter().filter(|&&m| m != f).map(|m| format!("{m:#x}")).collect();
        writeln!(out, "{f:#x}\t{status}\t{}", others.join(",")).map_err(|e| e.to_string())?;
        out.flush().ok();
    }
    Ok(())
}
