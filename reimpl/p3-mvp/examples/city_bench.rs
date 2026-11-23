//! Cronómetro y huella de la vista de ciudad: `city_bench PARTIDA.pat ARCHIVO.cpr [--ciudades todas|N,M] [--relojes 0,1000] [--fx] [--repetir N]`.
//!
//! Por cada ciudad, reloj y variante de efectos imprime una línea con el tiempo de `Session::render_fx_rgba` y el SHA-256 de la
//! imagen RGBA y del JSON devuelto. Sirve para comprobar que una optimización no cambia ni un píxel (se comparan las líneas
//! sin el campo `ms`) y para medir la apertura de la ciudad.
use p3_mvp::{Effects, Session};
use sha2::{Digest, Sha256};

fn hex(d: &[u8]) -> String {
    d.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let opt = |name: &str| a.iter().position(|x| x == name).and_then(|i| a.get(i + 1)).cloned();
    let t = std::time::Instant::now();
    let save = std::fs::read(&a[1]).unwrap();
    let cpr = p3_assets::cpr::Bytes::map_file(&a[2]).unwrap();
    let mut s = Session::open(&save, cpr).unwrap();
    eprintln!("abrir {:.1} ms", t.elapsed().as_secs_f64() * 1e3);
    let towns = s.sim.economy.world.towns.len();
    let cities: Vec<usize> = match opt("--ciudades").as_deref() {
        None => vec![s.city],
        Some("todas") => (0..towns).collect(),
        Some(l) => l.split(',').map(|v| v.parse().unwrap()).collect(),
    };
    let clocks: Vec<u32> = opt("--relojes").map_or(vec![0], |l| l.split(',').map(|v| v.parse().unwrap()).collect());
    let repeat: usize = opt("--repetir").map_or(1, |v| v.parse().unwrap());
    let mut variants = vec![("sin", Effects::default())];
    if a.iter().any(|x| x == "--fx") {
        variants.push(("nieve", Effects { weather: Some((true, 400)), seed: 7, ..Default::default() }));
        variants.push(("lluvia", Effects { weather: Some((false, 300)), seed: 3, conductor: true, ..Default::default() }));
        let cracks = s.building_cells().into_iter().take(6).map(|(x, y)| (x, y, 3)).collect();
        variants.push(("grietas", Effects { cracks, seed: 1, ..Default::default() }));
    }
    for &city in &cities {
        s.city = city;
        for &clock in &clocks {
            for (label, fx) in &variants {
                for _ in 0..repeat {
                    let t = std::time::Instant::now();
                    match s.render_fx_rgba(clock, fx) {
                        Ok((v, (w, h, rgba))) => {
                            let ms = t.elapsed().as_secs_f64() * 1e3;
                            println!("ciudad={city} reloj={clock} fx={label} {w}x{h} rgba={} json={} ms={ms:.1}", hex(&Sha256::digest(&rgba)), hex(&Sha256::digest(v.to_string().as_bytes())));
                        }
                        Err(e) => println!("ciudad={city} reloj={clock} fx={label} error={e}"),
                    }
                }
            }
        }
    }
}
