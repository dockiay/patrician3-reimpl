//! Lista barcos de un .pat: índice, dueño, casco (+0xE), tripulación (+0x40), estado (+0x134), ciudad (+0x39), convoy (+0x6?).
fn main(){
    let p=std::env::args().nth(1).expect("ruta .pat");
    let m=p3_save::model::SaveModel::parse(&std::fs::read(&p).unwrap()).unwrap();
    for i in 0..m.ships.ships.len(){
        let Ok(s)=m.ships.ship_image(i) else {continue};
        let name_pos=s.windows(4).position(|w|w==b"Rein");
        let w=|o:usize|u16::from_le_bytes([s[o],s[o+1]]);
        if s[0]==0 || name_pos.is_some(){
            println!("{i:3} dueño {} casco {} trip {} est {} ciudad {} conv {} +8 {} +6 {} nombre@{:?} +3f {}",s[0],s[0xe],w(0x40),w(0x134),s[0x39],w(4),w(8),w(6),name_pos,s[0x3f]);
        }
    }
}
