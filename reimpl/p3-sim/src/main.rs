use p3_sim::{Report, Sim, Summary};

fn print(label: &str, s: &Summary) {
    println!("{label}: tick {} ({:02}/{:02} año {}) oro comerciantes {} dinero ciudades {} población {:?} hash {}", s.tick, s.day, s.month + 1, s.year, s.merchants_gold, s.town_money, s.population, &s.hash[..16]);
}

fn main() {
    let mut raw: Vec<String> = std::env::args().skip(1).collect();
    // `--out ARCHIVO.pat`: guarda el estado final (nunca sobrescribe un archivo existente) y comprueba que se vuelve a cargar.
    let out = raw.iter().position(|a| a == "--out").map(|i| {
        let path = raw.get(i + 1).cloned().unwrap_or_else(|| {
            eprintln!("--out necesita un archivo");
            std::process::exit(2)
        });
        raw.drain(i..=i + 1);
        path
    });
    // La partida original no serializa el estado del CRT. El usuario lo aporta
    // explícitamente para reproducir y continuar los consumidores conectados.
    let crt_state = raw.iter().position(|a| a == "--crt-state").map(|i| {
        let value = raw.get(i + 1).cloned().unwrap_or_else(|| {
            eprintln!("--crt-state necesita un entero de 32 bits (decimal o 0x hexadecimal)");
            std::process::exit(2)
        });
        raw.drain(i..=i + 1);
        let parsed = if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
            u32::from_str_radix(hex, 16)
        } else { value.parse::<u32>() };
        parsed.unwrap_or_else(|_| {
            eprintln!("--crt-state no es un entero de 32 bits: {value}");
            std::process::exit(2)
        })
    });
    // Estos dos campos no se serializan en S05 y son necesarios para
    // reproducir el planificador de fiestas entre ejecuciones.
    let celebration = raw.iter().position(|a| a == "--celebration-state").map(|i| {
        let value=raw.get(i+1).cloned().unwrap_or_else(||{eprintln!("--celebration-state necesita NEXT:TOWN");std::process::exit(2)});
        raw.drain(i..=i+1);
        let parsed=value.split_once(':').and_then(|(next,town)|Some((next.parse::<u32>().ok()?,town.parse::<u8>().ok()?)));
        parsed.unwrap_or_else(||{eprintln!("--celebration-state inválido: usa NEXT:TOWN con enteros decimales");std::process::exit(2)})
    });
    let investigation_local=if let Some(i)=raw.iter().position(|a|a=="--investigation-local"){raw.remove(i);true}else{false};
    // `--exe Patrician3.exe`: textos de la elección del Ältermann del ejecutable inglés verificado.
    let exe = raw.iter().position(|a| a == "--exe").map(|i| {
        let path = raw.get(i + 1).cloned().unwrap_or_else(|| {
            eprintln!("--exe necesita el ejecutable inglés");
            std::process::exit(2)
        });
        raw.drain(i..=i + 1);
        path
    });
    let fleet_player = raw.iter().position(|a| a == "--fleet-player").map(|i| {
        let value=raw.get(i+1).cloned().unwrap_or_else(||{eprintln!("--fleet-player necesita un identificador de comerciante");std::process::exit(2)});
        raw.drain(i..=i+1);
        value.parse::<u16>().unwrap_or_else(|_|{eprintln!("--fleet-player inválido: {value}");std::process::exit(2)})
    });
    let mut args = raw.into_iter();
    let Some(path) = args.next() else {
        eprintln!("uso: p3-sim GUARDADO.pat [DÍAS] [ARCHIVO.cpr] [--crt-state ESTADO] [--celebration-state NEXT:TOWN] [--investigation-local] [--fleet-player ID] [--exe ORIGINAL.exe] [--out NUEVO.pat]");
        std::process::exit(2);
    };
    let days: u32 = args.next().map(|d| d.parse().expect("DÍAS debe ser un número")).unwrap_or(30);
    let file = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut sim = Sim::from_save_bytes(&file).unwrap_or_else(|e| panic!("{e}"));
    if let Some(path) = &exe { let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}")); sim.configure_election_texts(&bytes).unwrap_or_else(|e| panic!("textos de elección: {e}")); sim.configure_hunt_texts(&bytes).unwrap_or_else(|e| panic!("textos de caza: {e}")); sim.configure_discovery_locations(&bytes).unwrap_or_else(|e| panic!("posiciones de centros comerciales: {e}")); println!("textos de la elección del Ältermann cargados"); }
    if let Some(state) = crt_state { sim.configure_crt_state(state); println!("estado CRT inicial aportado: 0x{state:08x}"); }
    if let Some((next,town))=celebration{sim.host.celebration_state=Some(p3_core::economy::celebration::CelebrationState{next,town});println!("estado fiestas inicial aportado: {next}:{town}");}
    sim.host.investigation_local_mode=investigation_local;
    if investigation_local{println!("modo local de investigación aportado");}
    if let Some(archive)=args.next(){let bytes=std::fs::read(&archive).unwrap_or_else(|e|panic!("{archive}: {e}"));sim.configure_pirate_name_archives(&[&bytes]).unwrap_or_else(|e|panic!("recursos: {e}"));sim.configure_alderman_map_archives(&[&bytes]).unwrap_or_else(|e|panic!("mapa: {e}"));if let Some(player)=fleet_player{sim.configure_hanse_fleet_archives(&[&bytes],"scripts/Namen_eng.ini",false,player).unwrap_or_else(|e|panic!("flota: {e}"));}}
    else if fleet_player.is_some(){eprintln!("--fleet-player necesita un archivo CPR");std::process::exit(2)}
    if let Some(player)=fleet_player{let model=p3_save::model::SaveModel::parse(&file).unwrap_or_else(|e|panic!("modelo: {e:?}"));let count=sim.resume_saved_fleet(&model,u32::from(player)).unwrap_or_else(|e|{eprintln!("no se puede reconstruir la flota: {e}");std::process::exit(1)});println!("flota reconstruida: {count} barcos o líderes en movimiento");}
    else {println!("flota no conectada: para intentar reconstruirla, aporta CPR y --fleet-player ID");}
    let (d, m, y, doy) = sim.saved_calendar;
    println!("{path}: {} ciudades, {} comerciantes, {} oficinas, calendario guardado {d:02}/{:02} año {y} (día {doy})", sim.economy.world.towns.len(), sim.economy.world.merchants.len(), sim.economy.world.offices.len(), m + 1);
    let Report { start, end, days, inactive, errors } = sim.advance_days(days);
    print("inicio", &start);
    print("final ", &end);
    println!("tras {days} días");
    if let Some(rng) = sim.host.investigation_rng { println!("estado CRT final: 0x{:08x} (la partida no lo guarda)", rng.state); }
    if let Some(state)=sim.host.celebration_state{println!("estado fiestas final: {}:{} (la partida no lo guarda)",state.next,state.town);}
    if !errors.is_empty() {
        eprintln!("fallos del avance: {} (primero: {})", errors.len(), errors[0]);
    }
    println!("sin sistema detrás (llamadas): {inactive:?}");
    if !errors.is_empty() {
        if out.is_some() { eprintln!("no se guarda el resultado porque el avance se detuvo con errores"); }
        std::process::exit(1);
    }
    if let Some(out) = out {
        if std::path::Path::new(&out).exists() {
            eprintln!("{out} ya existe; no se sobrescribe");
            std::process::exit(2);
        }
        let mut model = p3_save::model::SaveModel::parse(&file).unwrap_or_else(|e| panic!("modelo: {e:?}"));
        sim.persist_all(&mut model).unwrap_or_else(|e| panic!("guardado: {e}"));
        let bytes = model.write();
        let again = Sim::from_save_bytes(&bytes).unwrap_or_else(|e| panic!("el resultado no se recarga: {e}"));
        assert_eq!(again.idle.clock.tick, sim.idle.clock.tick, "reloj tras recargar");
        std::fs::write(&out, &bytes).unwrap_or_else(|e| panic!("{out}: {e}"));
        println!("guardado en {out} ({} bytes); recargado con el mismo reloj", bytes.len());
    }
}
