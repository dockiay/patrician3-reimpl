//! Dibuja un texto con la maqueta del original (`TextLayout` 0x004209E0/0x00420A10) a través del enlazador de interfaz.
//! Uso: `cargo run -p p3-mvp --example ui_text -- Patrician3.exe p2arch0_eng.cpr salida.png [texto]`
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exe = std::fs::read(&args[0]).map_err(|e| e.to_string())?;
    let cpr = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("{e:?}"))?;
    let text = args.get(3).cloned().unwrap_or_else(|| "\n\\r\\f1_Luebeck,  2. 9. 1345\n\n\\lTaxes for your trading offices and workshops last week amounted to 280\\C.\n".into());
    let mut mem = p3_mvp::ui_rt::PageMem::default();
    let mut rt = p3_mvp::ui_rt::Runtime::new(&mut mem, &exe, &archive, 420, 200)?;
    let layout = rt.new_object(&mut mem, 0x40);
    rt.call(&mut mem, 0x004209E0, layout, &[]);
    rt.surface = p3_mvp::ui_rt::Surface::new(420, 200, [0xE8, 0xD8, 0xB0, 0xFF]);
    // `0x00420A10(CString texto, x, y, ancho, alto, color)`, `ret 0x18`: el texto va por valor y lo libera el llamado.
    let s = rt.cstring(&mut mem, text.as_bytes());
    let y = rt.call(&mut mem, 0x00420A10, layout, &[s, 10, 10, 400, 400, 0xFF20_1810]);
    rt.surface.write_png(std::path::Path::new(&args[2]))?;
    println!("y final {y}; líneas {}", mem.u32_at(layout + 8));
    println!("portadas: {:x?}", rt.ported);
    println!("faltan: {:x?}", rt.missing);
    println!("faltan (función, llamante): {:x?}", rt.missing_from);
    println!("SGL sin sustituto: {:?}", rt.sgl_missing);
    println!("importaciones sin sustituto: {:?}", rt.import_missing);
    println!("llamadas {} abortado {} montón {:#x}", rt.calls, rt.aborted, rt.heap);
    Ok(())
}
