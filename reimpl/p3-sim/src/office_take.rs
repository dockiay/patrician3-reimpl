//! Frontera viva de extracción de materiales con recálculo de carga y convoy.
use p3_core::economy::{office_take,ship_trade::ShipCargo,trade::OfficeStock};

#[derive(Clone,Copy,Debug)]
pub struct ShipLink {pub next:u16,pub state:u16,pub convoy:u16}

/// Retirada directa del comerciante (0x4f5450), sin marcar barco ni convoy.
/// Trabaja sobre copias para no publicar una lista cíclica o carga inválida.
pub fn take_ships(cargo:&mut [ShipCargo],links:&[ShipLink],head:u16,town:u8,ware:usize,amount:i32)->Result<(i32,Vec<usize>),String>{
    if cargo.len()!=links.len() || ware>=24 || amount<0 {return Err("flota, mercancía o cantidad inválida".into())}
    let mut staged=cargo.to_vec();let mut seen=vec![false;cargo.len()];let mut changed=Vec::new();
    let mut left=amount;let mut cursor=head;
    while left>0 && usize::from(cursor)<staged.len(){
        let i=usize::from(cursor);if seen[i]{return Err("ciclo en lista de barcos".into())}seen[i]=true;
        let ship=&mut staged[i];let link=links[i];
        if ship.town==town && link.state<4 && link.state!=2 {
            let available=ship.stock[ware];if available<0{return Err("carga naval negativa".into())}
            let taken=available.min(left);ship.stock[ware]-=taken;left-=taken;
            ship.recompute_load();changed.push(i);
        }
        cursor=link.next;
    }
    cargo.clone_from_slice(&staged);Ok((left,changed))
}

/// Actualiza copias y publica todo junto; devuelve lo que no se ha podido retirar.
pub fn take(office:&mut OfficeStock,cargo:&mut [ShipCargo],links:&[ShipLink],convoys:&mut [u32],head:u16,ware:usize,amount:i32)->Result<(i32,Vec<usize>),String>{
    if cargo.len()!=links.len() || ware>=24 {return Err("flota o mercancía incompatible".into())}
    let mut ships=cargo.iter().zip(links).map(|(s,l)|office_take::Ship{town:s.town,state:l.state,next:l.next,stock:s.stock[ware]}).collect::<Vec<_>>();
    let mut stock=office.stock[ware];
    let (left,changed)=office_take::take(&mut stock,&mut ships,head,office.town,ware,amount).map_err(str::to_string)?;
    let mut staged=cargo.to_vec();let mut staged_convoys=convoys.to_vec();
    for &i in &changed {
        let ship=&mut staged[i];ship.stock[ware]=ships[i].stock;ship.dirty|=4;
        if let Some(flags)=staged_convoys.get_mut(usize::from(links[i].convoy)){*flags|=4}
        ship.recompute_load();
    }
    office.stock[ware]=stock;cargo.clone_from_slice(&staged);convoys.copy_from_slice(&staged_convoys);Ok((left,changed))
}

impl crate::Sim {
    /// Retira materiales del estado vivo; las escrituras de S05 las realiza `persist_construction_city`.
    pub fn withdraw_building_materials(&mut self,office_index:usize,ware:usize,amount:i32)->Result<i32,String>{
        if ware>=24 || amount<0{return Err("mercancía o cantidad inválida".into())}
        let mut office=self.economy.offices.get(office_index).ok_or("oficina ausente")?.clone();
        let saved=self.persisted.get(office_index).ok_or("oficina persistida ausente")?;
        if office.stock!=saved.stock.stock || office.owner!=saved.stock.owner || office.town!=saved.stock.town{return Err("copias de oficina desincronizadas".into())}
        let merchant=self.rep_merchants.get(usize::from(office.owner)).ok_or("comerciante ausente")?;
        if merchant.bytes.len()<0x10{return Err("comerciante truncado".into())}
        let head=u16::from_le_bytes([merchant.bytes[0xe],merchant.bytes[0xf]]);
        let count=self.economy.cargo_ships.len();
        if self.host.expedition_ship_images.len()!=count || self.host.navigation.as_ref().is_some_and(|n|n.snapshot.ships.len()!=count*0x180){return Err("flota incompleta".into())}
        let mut links=Vec::with_capacity(count);
        for i in 0..count {
            if self.host.expedition_ship_images[i].as_ref().is_some_and(|b|b.len()<0x180){return Err("imagen de barco truncada".into())}
            let image=if let Some(nav)=&self.host.navigation {nav.snapshot.ships.get(i*0x180..(i+1)*0x180)}else{self.host.expedition_ship_images[i].as_deref()};
            if let Some(b)=image {
                if b.len()<0x180{return Err("imagen de barco truncada".into())}
                let cargo=&self.economy.cargo_ships[i];
                if cargo.town!=b[0x39] || (0..24).any(|w|i32::from_le_bytes(b[0x54+4*w..0x58+4*w].try_into().unwrap())!=cargo.stock[w]) {return Err("carga naval desincronizada".into())}
                links.push(ShipLink{next:u16::from_le_bytes([b[4],b[5]]),state:u16::from_le_bytes([b[0x134],b[0x135]]),convoy:u16::from_le_bytes([b[8],b[9]])});
            } else {links.push(ShipLink{next:u16::MAX,state:u16::MAX,convoy:u16::MAX})}
        }
        if office.stock[ware]<amount && self.host.navigation.is_none(){
            let mut i=head;let mut seen=vec![false;count];
            while usize::from(i)<count {let k=usize::from(i);if seen[k]{return Err("ciclo en lista de barcos".into())}seen[k]=true;
                if self.host.expedition_ship_images[k].is_none(){return Err("barco enlazado sin imagen".into())}i=links[k].next;}
        }
        let mut cargo=self.economy.cargo_ships.clone();let mut flags=self.host.debt_convoy_flags.clone();
        if let Some(nav)=&self.host.navigation {
            if nav.snapshot.convoys.len()!=flags.len()*0x3c{return Err("convoyes incompletos".into())}
            for (i,flag) in flags.iter().enumerate(){let p=i*0x3c+0x18;if u32::from_le_bytes(nav.snapshot.convoys[p..p+4].try_into().unwrap())!=*flag{return Err("convoy desincronizado".into())}}
        }
        let previous_flags=flags.clone();let (left,changed)=take(&mut office,&mut cargo,&links,&mut flags,head,ware,amount)?;
        self.economy.offices[office_index]=office.clone();self.persisted[office_index].stock=office;
        for &i in &changed {
            let s=&cargo[i];
            let write=|image:&mut [u8]| {
                for w in 0..24{image[0x54+4*w..0x58+4*w].copy_from_slice(&s.stock[w].to_le_bytes())}
                image[0x3c..0x3e].copy_from_slice(&s.dirty.to_le_bytes());
                for (p,v) in [(0x114,s.cargo_value),(0x118,s.cargo_weight),(0x11c,s.equipment_weight),(0x120,s.gun_power)]{image[p..p+4].copy_from_slice(&v.to_le_bytes())}
            };
            if let Some(image)=self.host.expedition_ship_images.get_mut(i).and_then(Option::as_mut){write(image)}
            if let Some(nav)=self.host.navigation.as_mut(){write(&mut nav.snapshot.ships[i*0x180..(i+1)*0x180])}
            if !self.host.debt_changed_ships.contains(&i){self.host.debt_changed_ships.push(i)}
        }
        for (i,(&before,&after)) in previous_flags.iter().zip(&flags).enumerate(){if before!=after{
            if let Some(nav)=self.host.navigation.as_mut(){let p=i*0x3c+0x18;nav.snapshot.convoys[p..p+4].copy_from_slice(&after.to_le_bytes())}
            if !self.host.debt_changed_convoys.contains(&i){self.host.debt_changed_convoys.push(i)}
        }}
        self.economy.cargo_ships=cargo;self.host.debt_convoy_flags=flags;Ok(left)
    }
}
