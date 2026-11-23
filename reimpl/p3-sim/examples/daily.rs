fn main() {
    let f = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();
    let mut sim = p3_sim::Sim::from_save_bytes(&f).unwrap();
    for d in 0..12 {
        let s = sim.summary();
        let t = &sim.economy.world.towns[0];
        println!("día {d}: pop {:?} t0 pop {:?} flags {:x} 0x77E.. {:?}", s.population, [t.pop(0), t.pop(1), t.pop(2), t.pop(3)], t.flags(), &t.0[0x770..0x790]);
        sim.advance_days(1);
    }
}
