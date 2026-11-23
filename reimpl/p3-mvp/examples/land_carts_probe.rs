//! Lista los transportes terrestres (tarea 0x0C) de una partida y los carros que `0x004D8F00` crearía en cada ciudad.
//! Uso: `cargo run --release -p p3-mvp --example land_carts_probe -- PARTIDA.pat CPR`
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let save = std::fs::read(&a[1]).unwrap();
    let cpr = std::fs::read(&a[2]).unwrap();
    let s = p3_mvp::Session::open(&save, cpr).unwrap();
    let sch = &s.sim.host.council_scheduler;
    let tick = s.sim.economy.world.tick;
    println!("tick {tick} capacidad {} cabeza {}", sch.capacity, sch.head);
    for (i, r) in sch.records.iter().enumerate() {
        if r.kind == 0x0C {
            println!("  tarea {i}: due {} de {} a {} +0xA {} +0xB {:#x} fase {} progreso {} carga {:02x?}", r.due, r.u8_at(8), r.u8_at(9), r.u8_at(0xA), r.u8_at(0xB), r.u8_at(0x15), r.u8_at(0x16), r.payload);
        }
    }
    let ids = &s.sim.host.expedition_town_ids;
    for (i, _) in s.sim.economy.world.towns.iter().enumerate() {
        if let Some(p) = p3_core::economy::land_transport_send::land_partner(&s.sim.economy.world.towns, ids, i) {
            let offices: Vec<_> = s.sim.economy.offices.iter().filter(|o| usize::from(o.town) == i).map(|o| o.owner).collect();
            println!("ciudad {i} (id {}) unida por tierra con {p} (id {}); oficinas de {offices:?}; jugador {}", ids[i], ids[usize::from(p)], s.owner);
        }
    }
    for town in 0..s.sim.economy.world.towns.len() as u8 {
        let carts = s.land_transport_carts(town);
        if !carts.is_empty() {
            println!("ciudad {town}: carros {carts:?}");
        }
    }
}
