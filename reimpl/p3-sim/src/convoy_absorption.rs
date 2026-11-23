//! Frontera de C10 para la absorción C07 y sus efectos de mundo.
use p3_core::ships::{convoy_absorb::{self,AbsorbHost,AbsorbWorld},tick::{ConvoyHost,TickWorld}};
use p3_save::pirate_bands::PirateBand;

#[derive(Clone,Debug)]
pub struct AbsorptionState {
    pub bands:Vec<Option<PirateBand>>,
    pub day:u16,
    pub network_client:u32,
    pub operations:Vec<p3_core::pump::Operation>,
}

struct Effects { operations:Vec<p3_core::pump::Operation>,released:Option<u16>,changed:bool,error:Option<String> }
impl AbsorbHost for Effects {
    fn ui(&mut self,_:u32,_:u32){self.error=Some("interfaz de absorción no configurada".into())}
    fn ship_refresh(&mut self,_:u16){self.error=Some("mejora naval sin imagen".into())}
    fn ship_refresh_with_image(&mut self,_:u16,ship:&mut [u8]){
        if let Err(e)=crate::absorb_ship_refresh::refresh(ship,1){self.error=Some(e.to_string())}
    }
    fn event(&mut self,block:[u32;5]){self.operations.push(p3_core::pump::Operation(block))}
    fn release(&mut self,convoy:u16){self.released=Some(convoy)}
    fn object_changed(&mut self){self.changed=true}
}

/// Debe llamarse sobre el mundo provisional del tick. El llamante publica
/// flota, banda y operaciones juntos solo si termina el tick completo.
pub fn absorb(w:&mut TickWorld,object:u16,convoy:u16,state:&mut AbsorptionState,host:&mut dyn ConvoyHost)->Result<(),String>{
    let r16=|v:&[u8],at:usize|u16::from_le_bytes([v[at],v[at+1]]);
    if w.remove.manager.len()<0xfa{return Err("gestor naval truncado".into())}
    let ships=r16(w.remove.manager,0xf4);let captains=r16(w.remove.manager,0xf2);let convoys=r16(w.remove.manager,0xf6);
    if convoy>=convoys||w.remove.ships.len()<usize::from(ships)*0x180||w.remove.convoys.len()<usize::from(convoys)*0x3c{return Err("flota de absorción inválida".into())}
    let band=state.bands.get_mut(usize::from(object)).and_then(Option::as_mut).ok_or("banda de absorción ausente")?;
    band.validate().map_err(str::to_owned)?;
    let mut current=r16(w.remove.convoys,usize::from(convoy)*0x3c+0xa);let mut seen=vec![false;usize::from(ships)];
    while current<ships{let i=usize::from(current);if seen[i]{return Err("cadena de absorción cíclica".into())}seen[i]=true;current=r16(w.remove.ships,i*0x180+6)}
    let mut image=[0u8;0x18];image[8..].copy_from_slice(&band.header);
    let mut effects=Effects{operations:Vec::new(),released:None,changed:false,error:None};
    let mut world=AbsorbWorld{ships:w.remove.ships,convoys:w.remove.convoys,ship_count:ships,captain_count:captains,ui:0,suppress_events:state.network_client};
    convoy_absorb::absorb_convoy(&mut image,&mut world,u32::from(convoy),&mut effects).map_err(str::to_owned)?;
    if let Some(e)=effects.error{return Err(e)}
    if effects.released!=Some(convoy)||!effects.changed{return Err("absorción incompleta".into())}
    // release no ve ya barcos; la mejora y los paquetes precedentes se han
    // ejecutado/encolado. La guarida es el último efecto de la llamada.
    p3_core::ships::convoy::release_convoy(w,convoy,host);
    band.header.copy_from_slice(&image[8..]);
    crate::absorb_hut::build_hut(band,convoys,w.time,state.day)?;
    state.operations.extend(effects.operations);
    Ok(())
}
