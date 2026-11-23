//! Decisiones de vivienda y gremio del turno de comerciante C06 sobre datos vivos.
use p3_core::{
    ai::{guild::{self, GuildHost, GuildMerchant, GuildOffice}, housing::{self, HousingEnv, HousingHost, HousingRecord, HousingTown}, reputation::Merchant},
    buildings::Site, economy::world::EconomyWorld, pump::Operation,
};

fn u16_at(b:&[u8],at:usize)->u16 { u16::from_le_bytes([b[at],b[at+1]]) }
fn u32_at(b:&[u8],at:usize)->u32 { u32::from_le_bytes(b[at..at+4].try_into().expect("comerciante validado")) }

struct HousingBridge<'a> {
    world:&'a EconomyWorld,
    sites:&'a [Site],
    offices:&'a [Vec<u8>],
    chain:&'a [usize],
    operations:Vec<Operation>,
    error:Option<String>,
}
impl HousingHost for HousingBridge<'_> {
    fn find_or_create_office(&mut self,owner:u32,town:u8)->Option<u16>{
        let Some(city)=self.world.towns.get(usize::from(town))else{self.error=Some("ciudad de vivienda ausente".into());return None};
        let mut at=city.u16(0x784);let mut seen=vec![false;self.offices.len()];
        while usize::from(at)<self.offices.len(){let i=usize::from(at);if seen[i]{self.error=Some("cadena urbana de oficinas cíclica".into());return None}seen[i]=true;let office=&self.offices[i];
            if u32::from(u16_at(office,0x2c4))==owner{if !self.chain.contains(&i){self.error=Some("oficina de vivienda fuera de la cadena del comerciante".into());return None}return Some(u16_at(office,0x2d0))}at=u16_at(office,0x2ca);
        }
        self.error=Some("oficina de vivienda ausente de la ciudad".into());None
    }
    fn find_nearest_cell(&mut self,town:u8,x:&mut u8,y:&mut u8,kind:u8,extra:u8,flag:bool)->bool{
        let i=usize::from(town);let (Some(t),Some(s))=(self.world.towns.get(i),self.sites.get(i))else{self.error=Some("plano de vivienda ausente".into());return false};
        if let Err(e)=crate::lender::validate(t,s,self.world.merchant_count){self.error=Some(e);return false}
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(||s.find_nearest_cell_for_type(t,x,y,i32::from(kind),extra,flag,0xe0))).unwrap_or_else(|_|{self.error=Some("búsqueda de vivienda fuera del plano".into());false})
    }
    fn enqueue_operation(&mut self,r:&[u32;5]){self.operations.push(Operation(*r))}
}
struct GuildBridge(Vec<Operation>);
impl GuildHost for GuildBridge { fn enqueue_operation(&mut self,r:&[u32;5]) { self.0.push(Operation(*r)) } }

/// Prepara las operaciones en el mismo orden que el turno. No modifica el mundo.
pub(crate) fn plan(world:&EconomyWorld,sites:&[Site],images:&[Vec<u8>],merchant:&Merchant,id:u16,housing:bool,guild:bool)->Result<Vec<Operation>,String>{
    if world.towns.len()>40||images.len()!=world.offices.len()||images.iter().any(|o|o.len()!=0x44c)||sites.len()<world.towns.len()||merchant.bytes.len()<0x650{return Err("datos de decisiones IA incompletos".into())}
    let mut chain=Vec::new();let mut seen=vec![false;images.len()];let mut at=merchant.u16(0xc);
    while usize::from(at)<images.len(){let i=usize::from(at);if seen[i]{return Err("cadena de oficinas IA cíclica".into())}seen[i]=true;
        if u16_at(&images[i],0x2c4)!=id||usize::from(images[i][0x2c6])>=world.towns.len(){return Err("oficina de decisiones IA incoherente".into())}chain.push(i);at=u16_at(&images[i],0x2c8);
    }
    let links=images.iter().map(|o|GuildOffice{town:o[0x2c6],next:u16_at(o,0x2c8)}).collect::<Vec<_>>();
    let mut operations=Vec::new();
    if housing {
        let mut towns=Vec::with_capacity(world.towns.len());
        for (i,town) in world.towns.iter().enumerate(){let site=&sites[i];let count=usize::from(town.u16(0x776));if site.records.len()<count*8{return Err("registros de vivienda truncados".into())}
            let records=site.records[..count*8].chunks_exact(8).map(|r|HousingRecord{x:r[0],y:r[1],kind:r[3],next:u16_at(r,4)}).collect::<Vec<_>>();
            let mut heads=vec![town.u16(0x762),town.u16(0x764)];heads.extend(chain.iter().filter(|&&office|usize::from(images[office][0x2c6])==i).map(|&office|u16_at(&images[office],0x2d0)));
            for mut head in heads {let mut visited=vec![false;count];while usize::from(head)<count{let n=usize::from(head);if visited[n]{return Err("lista de edificios de vivienda cíclica".into())}visited[n]=true;head=records[n].next;}}
            towns.push(HousingTown{records,head:town.u16(0x762),head2:town.u16(0x764),size:town.u8(0x774) as i8,demand:std::array::from_fn(|k|town.i32(0x2d8+4*k)),divisor:std::array::from_fn(|k|town.u16(0x2f8+2*k)),start:(town.u8(0x800),town.u8(0x801)),flag_10:town.flags()&0x10!=0});
        }
        let env=HousingEnv{town_count:world.towns.len() as u32,flag_2a:world.world_8a!=0,extra_small:0x20,extra_large:0xe0,towns:&towns,offices:&links,office_head:merchant.u16(0xc)};
        let mut bridge=HousingBridge{world,sites,offices:images,chain:&chain,operations:Vec::new(),error:None};housing::choose_housing(u32::from(id),&env,&mut bridge);
        if let Some(error)=bridge.error{return Err(error)}operations.extend(bridge.operations);
    }
    if guild {
        let person=GuildMerchant{gold:merchant.i32(0),flags:merchant.bytes[8],office_head:merchant.u16(0xc),home:merchant.bytes[0x19],guild_mask:u32_at(&merchant.bytes,0x468),income:merchant.i32(0x46c),reputation:(0..40).map(|i|f32::from_bits(u32_at(&merchant.bytes,0x2fc+4*i))).collect()};
        let owners=world.towns.iter().map(|t|t.u8(0x2c0)).collect::<Vec<_>>();let mut bridge=GuildBridge(Vec::new());guild::try_join_guild(&person,u32::from(id),world.towns.len() as u32,&owners,&links,world.world_8a!=0,&mut bridge);operations.extend(bridge.0);
    }
    Ok(operations)
}
