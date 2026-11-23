//! Construye una ventana del original y la dibuja a través del enlazador de interfaz, para ver qué falta.
//! Uso: `ui_window Patrician3.exe p2arch0_eng.cpr salida.png TAMAÑO CTOR [MÉTODO...]` (direcciones en hex; cada método
//! se llama con `ecx` = objeto y sin argumentos, en orden: p. ej. carga de maqueta y dibujo).
fn hex(s: &str) -> u32 {
    u32::from_str_radix(s.trim_start_matches("0x"), 16).expect("hex")
}
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exe = std::fs::read(&args[0]).map_err(|e| e.to_string())?;
    let cpr = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let archive = p3_assets::cpr::Archive::parse(&cpr).map_err(|e| format!("{e:?}"))?;
    let mut mem = p3_mvp::ui_rt::PageMem::default();
    let mut rt = p3_mvp::ui_rt::Runtime::new(&mut mem, &exe, &archive, 800, 600)?;
    rt.surface = p3_mvp::ui_rt::Surface::new(800, 600, [0x30, 0x30, 0x30, 0xFF]);
    // `P3_CALLS=N`: límite de llamadas del enlazador (por defecto 2 millones; las escenas de edificio cargan muchas texturas).
    if let Some(n) = std::env::var("P3_CALLS").ok().and_then(|v| v.parse().ok()) {
        rt.call_limit = n;
    }
    // Con `P3_SAVE`, el estado de esa partida en las direcciones del original (puente C08 de p3-sim).
    let mut current_town = None;
    if let Some(save) = std::env::var_os("P3_SAVE") {
        let save = std::fs::read(save).map_err(|e| e.to_string())?;
        let session = p3_mvp::Session::open_with_paused_battle_and_crt(&save, cpr.clone(), 1)?;
        println!("partida sembrada: {} bytes", rt.seed_session(&mut mem, &session)?);
        // Ciudad a la vista: la de la oficina del jugador, o `P3_TOWN=N`.
        current_town = Some(std::env::var("P3_TOWN").ok().and_then(|t| t.parse().ok()).unwrap_or(session.city as u32));
    }
    // Con `P3_SCENE=1`, la escena global `[0x006E51AC]` creada como en el arranque del juego.
    if std::env::var_os("P3_SCENE").is_some() {
        let scene = rt.create_scene(&mut mem);
        println!("escena creada en {scene:#x}");
        // Las ventanas de edificio toman la ciudad de `escena+0xC324` al abrirse (p. ej. `0x5DDB3E`).
        if let Some(town) = current_town {
            mem.write(scene + 0xc324, &town.to_le_bytes());
            println!("ciudad a la vista: {town}");
        }
    } else {
        // Sin escena, al menos los estilos de botón del arranque: los menús laterales copian sus botones de ellos.
        rt.load_button_styles(&mut mem);
    }
    // Con `P3_HOST=1`, una pantalla contenedora (`screen_window`, ctor `0x4B4920`) de 800x600 apilada en la lista de
    // pantallas: las ventanas que se abren se cuelgan de ella y su dibujo (`^DIRECCIÓN`) recorre los hijos.
    let mut host_screen = 0;
    if std::env::var_os("P3_HOST").is_some() {
        host_screen = rt.new_object(&mut mem, 0x400);
        rt.call(&mut mem, 0x004B_4920, host_screen, &[]);
        rt.call(&mut mem, 0x0040_24B0, host_screen, &[800, 600]);
        let r = rt.call(&mut mem, 0x004B_90E0, 0x006D_A5F0, &[host_screen, 0]);
        println!("pantalla contenedora en {host_screen:#x}, apilada -> {r:#x}");
    }
    let mut obj = rt.new_object(&mut mem, hex(&args[3]));
    rt.call(&mut mem, hex(&args[4]), obj, &[]);
    let mut objects = vec![obj];
    // Cada paso: `DIRECCIÓN[:arg,arg...]` (llamada con ecx = objeto) o `=DESPLAZAMIENTO:valor` (byte en el objeto).
    for m in &args[5..] {
        // Varios objetos: `new:TAMAÑO:CTOR` crea otro y lo hace el actual; `use:N` vuelve al N-ésimo (0 = el primero);
        // `glob:DIRECCIÓN` guarda el actual en un global del juego; `child` lo cuelga de la pantalla contenedora.
        if let Some(rest) = m.strip_prefix("new:") {
            let (size, ctor) = rest.split_once(':').expect("new:tamaño:ctor");
            obj = rt.new_object(&mut mem, hex(size));
            rt.call(&mut mem, hex(ctor), obj, &[]);
            objects.push(obj);
            continue;
        }
        if let Some(n) = m.strip_prefix("use:") {
            obj = objects[n.parse::<usize>().expect("use:N")];
            continue;
        }
        if let Some(addr) = m.strip_prefix("glob:") {
            mem.write(hex(addr), &obj.to_le_bytes());
            continue;
        }
        if m == "child" {
            rt.call(&mut mem, 0x004B_4E30, host_screen, &[obj]);
            continue;
        }
        if let Some(rest) = m.strip_prefix('=') {
            let (off, v) = rest.split_once(':').expect("=off:valor");
            mem.write(obj + hex(off), &[hex(v) as u8]);
            continue;
        }
        let (f, a) = m.split_once(':').unwrap_or((m, ""));
        if hex(f.trim_start_matches(['~', '^', 'v'])) == 0 {
            continue;
        }
        // Argumentos: decimales, hex (`0x..`) o `s=TEXTO` (un `CString` por valor, p. ej. el fichero de maqueta).
        let a: Vec<u32> = a
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| match x.strip_prefix("s=") {
                Some(text) => rt.cstring(&mut mem, text.as_bytes()),
                None => x.parse::<i64>().map_or_else(|_| hex(x), |v| v as u32),
            })
            .collect();
        // `~DIRECCIÓN`: método de la lista de pantallas global (`ecx = 0x006DA5F0`) en lugar de la ventana.
        let (f, this) = f.strip_prefix('~').map_or((f, obj), |f| (f, 0x006D_A5F0));
        // `^DIRECCIÓN`: método de la pantalla contenedora de `P3_HOST`.
        let (f, this) = f.strip_prefix('^').map_or((f, this), |f| (f, host_screen));
        // `vDESPLAZAMIENTO`: método virtual del objeto (p. ej. `v120`, abrir).
        let target = match f.strip_prefix('v') {
            Some(slot) => mem.u32_at(mem.u32_at(this) + hex(slot)),
            None => hex(f),
        };
        let r = rt.call(&mut mem, target, this, &a);
        println!("{m} -> {r:#x}");
    }
    // `P3_DUMP=dirección,palabras`: imprime palabras de la memoria al final (diagnóstico).
    if let Ok(d) = std::env::var("P3_DUMP") {
        let (a, n) = d.split_once(',').unwrap_or((&d, "8"));
        // `obj+DESPLAZAMIENTO` vuelca desde la ventana construida.
        let (a, n) = (if let Some(off) = a.strip_prefix("obj+") { obj + hex(off) } else { hex(a) }, n.parse::<u32>().unwrap_or(8));
        let words: Vec<String> = (0..n).map(|i| format!("{:#x}", mem.u32_at(a + 4 * i))).collect();
        println!("memoria {a:#x}: {}", words.join(" "));
    }
    rt.surface.write_png(std::path::Path::new(&args[2]))?;
    let drawn = rt.surface.rgba.chunks_exact(4).filter(|p| p != &[0x30, 0x30, 0x30, 0xFF]).count();
    println!("píxeles dibujados: {drawn}");
    println!("portadas: {}", rt.ported.len());
    println!("faltan (función, llamante): {:x?}", rt.missing_from);
    // `P3_ARGLOG=fichero`: añade las llamadas con argumentos mal deducidos (entrada de `tools/ui_rt_fix.py`).
    if let Some(path) = std::env::var_os("P3_ARGLOG") {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path).map_err(|e| e.to_string())?;
        for ((caller, ret), (real, deduced)) in &rt.arg_mismatch {
            writeln!(f, "{caller:#x} {ret:#x} {real} {deduced}").map_err(|e| e.to_string())?;
        }
    }
    println!("SGL sin sustituto: {:?}", rt.sgl_missing);
    println!("importaciones sin sustituto: {:?}", rt.import_missing);
    println!("llamadas {} abortado {}", rt.calls, rt.aborted);
    Ok(())
}
