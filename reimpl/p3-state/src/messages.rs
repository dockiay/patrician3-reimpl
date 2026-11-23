//! Entrega del aviso de ampliación de almacén en el pool de mensajes S05.
use p3_core::ai::reputation::Merchant;
use p3_save::tail::MessagePool;

/// Comando 0xF0: retira mapas 0x45 y completa máscaras 0x46 de un tesoro.
/// Recorre como máximo el número de ranuras, como 0x004D7030.
pub fn remove_treasure_letters(pool:&mut Vec<u8>,merchants:&mut [Merchant],merchant_count:u16,merchant:u32,entry:u32)->Result<(),String>{
    if merchant>=u32::from(merchant_count)||entry>=100{return Ok(())}
    let person=merchants.get_mut(merchant as usize).ok_or("comerciante del tesoro ausente")?;
    if person.bytes.len()<12{return Err("comerciante del tesoro truncado".into())}
    let parsed=MessagePool::parse(pool).map_err(|e|format!("pool: {e:?}"))?;
    let (x,y)=p3_core::mission_treasure::COORDINATES[entry as usize];let count=usize::from(parsed.slots);
    let mut records=(0..count).map(|i|parsed.letter(i).to_vec()).collect::<Vec<_>>();let mut free=parsed.field_04;let mut head=person.u16(0xa);let mut at=head;let mut previous=None;
    for _ in 0..count {
        let i=usize::from(at);if i>=count{break}let r=&records[i];if r.len()<16{return Err("carta del tesoro truncada".into())}
        let next=u16::from_le_bytes([r[6],r[7]]);let matching=u16::from_le_bytes([r[12],r[13]])==x&&u16::from_le_bytes([r[14],r[15]])==y;
        if r[4]==0x45&&matching{
            if let Some(prev)=previous{let record:&mut Vec<u8>=&mut records[prev];record[6..8].copy_from_slice(&next.to_le_bytes())}else{head=next}
            records[i][4]=0xff;records[i][6..8].copy_from_slice(&free.to_le_bytes());free=at;
        }else{if r[4]==0x46&&matching{records[i][11]|=0xfe}previous=Some(i)}at=next;
    }
    let mut output=pool[..p3_save::tail::MESSAGE_HEADER_LEN].to_vec();output[6..8].copy_from_slice(&free.to_le_bytes());for record in records{output.extend_from_slice(&record)}
    person.set_u16(0xa,head);*pool=output;Ok(())
}

/// Entrega una carta de la subasta de edificios al comerciante correspondiente.
pub fn add_building_auction_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    let offset=match record[4]{0x76|0x7d|0x7e=>0x38,0x7f=>0x39,_=>return Err("tipo de carta de subasta inválido".into())};
    let setting=*merchant.bytes.get(offset).ok_or("preferencia de cartas de subasta ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],record[4],record[8..16].try_into().unwrap(),setting!=5)
}

/// Noticia 0x7B, categoría 1, dirigida a un comerciante humano.
pub fn add_mayor_statue_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if record[4]!=0x7b{return Err("tipo de carta de estatua inválido".into())}
    if merchant.bytes.len()<12{return Err("comerciante truncado".into())}
    if merchant.u16(8)!=0{return Ok(false)}
    let setting=*merchant.bytes.get(0x38).ok_or("preferencia de cartas ausente")?;
    if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],record[4],record[8..16].try_into().unwrap(),setting!=5)
}

/// Resultado 0x24 de una fiesta humana, categoría de mensajes 2.
pub fn add_celebration(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,outcome:&p3_core::economy::celebration::HumanCelebration,enabled:bool)->Result<bool,String>{
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    let mut payload=[0u8;8];for(i,value)in [outcome.requested,outcome.guests,outcome.quality].into_iter().enumerate(){payload[i*2..i*2+2].copy_from_slice(&value.to_le_bytes())}
    add_inline_letter(pool,merchant,day,month,year,town,0x24,payload,enabled&&setting!=5)
}

/// Cartas 0x31/0x32 de devolución de barcos de un convoy disuelto.
pub fn add_convoy_disband_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,kind:u8,words:[u16;4],retention:&[u32;6])->Result<bool,String>{
    if !matches!(kind,0x31|0x32){return Err("tipo de carta de convoy inválido".into())}
    let setting=*merchant.bytes.get(0x38).ok_or("preferencia de cartas ausente")?;
    if setting>5{return Err("preferencia de cartas inválida".into())}
    let mut payload=[0;8];for (i,value) in words.into_iter().enumerate(){payload[i*2..i*2+2].copy_from_slice(&value.to_le_bytes())}
    add_inline_letter(pool,merchant,day,month,year,town,kind,payload,retention[usize::from(setting)]!=0)
}

/// Aviso 0x4D por la subida del salario de un capitán de barco.
pub fn add_captain_skill_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,notice:&p3_core::economy::captain_operations::SkillNotice)->Result<bool,String>{
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;
    if setting>5{return Err("preferencia de cartas inválida".into())}
    let mut payload=[0u8;8];for (i,value) in [notice.look_a,notice.look_b,notice.ship_name,notice.wage].into_iter().enumerate(){payload[i*2..i*2+2].copy_from_slice(&value.to_le_bytes())}
    add_inline_letter(pool,merchant,day,month,year,notice.town,0x4d,payload,setting!=5)
}

/// Aviso 0x52 de jubilación del capitán de un barco.
pub fn add_captain_retirement_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,notice:&p3_core::economy::captain_operations::RetirementNotice)->Result<bool,String>{
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;
    if setting>5{return Err("preferencia de cartas inválida".into())}
    let mut payload=[0u8;8];for (i,value) in [notice.look_a,notice.look_b,notice.ship_name,notice.merchant_name].into_iter().enumerate(){payload[i*2..i*2+2].copy_from_slice(&value.to_le_bytes())}
    add_inline_letter(pool,merchant,day,month,year,notice.town,0x52,payload,setting!=5)
}

/// Aviso 0x7C por la mejora del administrador de una oficina.
pub fn add_administrator_skill_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,notice:&p3_core::economy::captain_operations::AdministratorNotice)->Result<bool,String>{
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;
    if setting>5{return Err("preferencia de cartas inválida".into())}
    let mut payload=[0u8;8];for (i,value) in [notice.combined_wage,notice.look_a,notice.look_b].into_iter().enumerate(){payload[i*2..i*2+2].copy_from_slice(&value.to_le_bytes())}
    add_inline_letter(pool,merchant,day,month,year,notice.town,0x7c,payload,setting!=5)
}

/// Busca una carta 0x5F por su valor, siguiendo la cadena del comerciante
/// como `chain_has_type_5f_with_value` (0x004D7AC0). Una cadena cíclica se
/// rechaza: el original no terminaría en ese caso.
pub fn rank_message_pending(pool:&[u8], head:u16, value:u32)->Result<bool,String>{
    let parsed=MessagePool::parse(pool).map_err(|e|format!("pool: {e:?}"))?;
    let mut seen=vec![false;usize::from(parsed.slots)];
    let mut at=usize::from(head);
    while at<seen.len(){
        if seen[at]{return Err("cadena de cartas cíclica".into())}
        seen[at]=true;
        let letter=parsed.letter(at);
        if letter[4]==0x5f && u32::from(u16::from_le_bytes([letter[10],letter[11]]))==value{return Ok(true)}
        at=usize::from(u16::from_le_bytes([letter[6],letter[7]]));
    }
    Ok(false)
}

/// Inserta uno de los avisos diarios contrastados con `add_message` para un
/// destinatario directo y sin interfaz activa. El llamante resuelve el índice
/// del destinatario y publica el pool y el comerciante en una transacción.
pub fn add_merchant_daily_record(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if !matches!(record[4],0x0f|0x10|0x11|0x5e|0x5f|0x6d|0x6e|0x6f|0x70|0x80|0x82|0x84){return Err("tipo de carta diaria no contrastado".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],record[4],record[8..16].try_into().unwrap(),true)
}

/// Carta 0x34: barco perdido durante expedición, categoría 2 del original.
/// El receptor es humano y su preferencia +0x39 controla la retención.
pub fn add_expedition_lost(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if record[4]!=0x34{return Err("tipo de carta de expedición inválido".into())}
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;
    if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],0x34,record[8..16].try_into().unwrap(),setting!=5)
}

/// Oferta, dossier, aviso tardío o anuncio de boda, categoría 2.
pub fn add_matchmaker_offer(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if !matches!(record[4],0x21|0x22|0x2b|0x33){return Err("tipo de oferta inválido".into())}
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],record[4],record[8..16].try_into().unwrap(),setting!=5)
}

/// Regalos navales 0x25 de boda, categoría 2; conserva la preferencia personal.
pub fn add_wedding_gifts(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if record[4]!=0x25{return Err("tipo de carta de regalos inválido".into())}
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],record[4],record[8..16].try_into().unwrap(),setting!=5)
}

/// Aviso personal de cosecha `0x08`, categoría 2.
pub fn add_harvest_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if record[4]!=8{return Err("tipo de aviso de cosecha inválido".into())}let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],8,record[8..16].try_into().unwrap(),setting!=5)
}

/// Contrato `0x1A` o resultado `0x5A/0x5B` de suministros.
pub fn add_supply_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    let field=match record[4]{0x5a=>0x3a,0x1a|0x5b=>0x39,_=>return Err("tipo de aviso de suministros inválido".into())};let setting=*merchant.bytes.get(field).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],record[4],record[8..16].try_into().unwrap(),setting!=5)
}

/// Anuncio 0x81 del astillero, categoría 2, enviado a cada comerciante.
pub fn add_shipyard_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8)->Result<bool,String>{
    let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
    add_inline_letter(pool,merchant,day,month,year,town,0x81,[0;8],setting!=5)
}

/// Inserta un registro inline en la cadena global (destinatario -2) de
/// `add_message` 0x004D6530. Mantiene la lista y el pool de forma atómica.
pub fn add_global_inline_letter(pool:&mut Vec<u8>,day:u8,month:u8,year:u16,record:&[u8;16])->Result<(),String>{
 if !matches!(record[4],0x06|0x09|0x0c|0x0d|0x1c|0x23|0x2a|0x35|0x38|0x53|0x55|0x56|0x58|0x59|0x6f|0x7b){return Err("tipo de carta global no contrastado".into())}
 let mut staged=pool.clone();let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let mut free=usize::from(parsed.field_04);let count=usize::from(parsed.slots);
 if free>=count{let new=count.checked_add(200).filter(|&n|n<=u16::MAX as usize).ok_or("crecimiento del pool fuera de rango")?;free=count;staged[4..6].copy_from_slice(&(new as u16).to_le_bytes());for i in count..new{let mut slot=[0u8;16];slot[1]=0x80;slot[2..6].fill(0xff);slot[6..8].copy_from_slice(&((i+1)as u16).to_le_bytes());staged.extend_from_slice(&slot);}}
 let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let range=parsed.letters[free].range.clone();if range.len()!=16||parsed.letters[free].kind!=0xff{return Err("ranura libre de mensajes inválida".into())}
 let next=u16::from_le_bytes([staged[range.start+6],staged[range.start+7]]);let head=parsed.field_08;let mut letter=*record;letter[0]=day;letter[1]=month|0x80;letter[2..4].copy_from_slice(&year.to_le_bytes());letter[6..8].copy_from_slice(&head.to_le_bytes());staged[range].copy_from_slice(&letter);staged[6..8].copy_from_slice(&next.to_le_bytes());staged[8..10].copy_from_slice(&(free as u16).to_le_bytes());*pool=staged;Ok(())
}

/// Entrega la carta 0x60 de `add_message` con interfaz desactivada.
/// `enabled` representa la preferencia de mensajes de la categoría 2 del comerciante.
/// El pool crece por 200 ranuras como el original; el error no modifica el estado.
pub fn add_office_expansion(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x60,[0;8],enabled)
}

/// Carta 0x54 por la baja de un barco pirata con capitán. La categoría 4 se entrega a humanos sin filtro de preferencias.
/// Carta 0x54 con la carga completa del registro del combate (`record[8..16]`): en el abordaje
/// (`ships::capture`) el original reutiliza el mensaje de la carta 0x2A y conserva su `+14`
/// (byte 3 del capitán), que también se guarda (ConvoyMar, día 176).
pub fn add_pirate_captain_record(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16])->Result<bool,String>{
    if record[4]!=0x54{return Err("tipo de carta de capitán inválido".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],0x54,record[8..16].try_into().unwrap(),true)
}
pub fn add_pirate_captain_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,look:u16,captain:u16,ship:u16)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..2].copy_from_slice(&look.to_le_bytes());payload[2..4].copy_from_slice(&captain.to_le_bytes());payload[4..6].copy_from_slice(&ship.to_le_bytes());add_inline_letter(pool,merchant,day,month,year,town,0x54,payload,true)
}

/// Carta de botadura (tipo 0), categoría de preferencias 2.
pub fn add_ship_launch(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,ship:u16,class:u16,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..2].copy_from_slice(&ship.to_le_bytes());payload[2..4].copy_from_slice(&class.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0,payload,enabled)
}

/// Carta oficial 0x85 por el pago de una petición de suministros.
pub fn add_siege_supply_payment(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,payment:&p3_core::economy::siege::SupplyPayment,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..2].copy_from_slice(&(u16::from(payment.without_bonus)).to_le_bytes());payload[2..4].copy_from_slice(&payment.amount.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,payment.home,0x85,payload,enabled)
}

/// Carta oficial 0x64 de una petición de suministros de asedio.
pub fn add_siege_supply_request(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16],enabled:bool)->Result<bool,String>{
    if record[4]!=0x64{return Err("tipo de petición de asedio inválido".into())}
    add_inline_letter(pool,merchant,day,month,year,record[5],0x64,record[8..16].try_into().unwrap(),enabled)
}

/// Carta de obra terminada (tipo 3), categoría de preferencias 2.
pub fn add_building_finished(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,kind:u16,record:u16,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..2].copy_from_slice(&kind.to_le_bytes());payload[2..4].copy_from_slice(&record.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,3,payload,enabled)
}

/// Factura de muralla 0x2F, categoría de preferencias 1.
pub fn add_wall_bill(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,w8:u16,wa:u16,wc:u16,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[0..2].copy_from_slice(&w8.to_le_bytes());payload[2..4].copy_from_slice(&wa.to_le_bytes());payload[4..6].copy_from_slice(&wc.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x2f,payload,enabled)
}

/// Carta 0x6B (categoría 1): decisión del concejo.
pub fn add_council_decision(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,a:u32,b:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..4].copy_from_slice(&a.to_le_bytes());payload[4..].copy_from_slice(&b.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x6b,payload,enabled)
}

/// Carta 0x6C (categoría 1): parte y total del recargo.
pub fn add_extra_tax(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,share:u32,total:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..4].copy_from_slice(&share.to_le_bytes());payload[4..].copy_from_slice(&total.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x6c,payload,enabled)
}

/// Aviso 0x13 por embargo íntegro en efectivo.
pub fn add_debt_cash(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,amount:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[3]=1;payload[4..].copy_from_slice(&amount.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x13,payload,enabled)
}

/// Aviso 0x12: fecha indicada en la deuda, no la fecha de entrega.
pub fn add_debt_notice(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,amount:u16,original_tick:u32,enabled:bool)->Result<bool,String>{
    let date=p3_core::calendar::Calendar::for_tick(original_tick);let mut payload=[0u8;8];
    payload[..2].copy_from_slice(&amount.to_le_bytes());payload[2..4].copy_from_slice(&date.year.to_le_bytes());
    payload[4]=date.month.wrapping_add(1);payload[6]=date.day;
    add_inline_letter(pool,merchant,day,month,year,town,0x12,payload,enabled)
}

/// Primera carta 0x77: petición por recargo injusto.
pub fn add_hanse_tax_petition(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,target:u8,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[0]=target;payload[1]=1;
    add_inline_letter(pool,merchant,day,month,year,town,0x77,payload,enabled)
}

pub fn add_hanse_petition(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x77,payload,enabled)
}

/// Segundo plazo de la petición 0x77, dirigido al representante.
pub fn add_hanse_reminder(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[6]=1;add_inline_letter(pool,merchant,day,month,year,town,0x77,payload,enabled)
}

/// Última carta 0x77, dirigida al representante.
pub fn add_hanse_reprimand(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[6]=2;add_inline_letter(pool,merchant,day,month,year,town,0x77,payload,enabled)
}

/// Aviso 0x26 de cobro por deudor externo.
pub fn add_debt_credit(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,amount:u16,original_tick:u32,enabled:bool)->Result<bool,String>{
    let date=p3_core::calendar::Calendar::for_tick(original_tick);let mut payload=[0u8;8];
    payload[..2].copy_from_slice(&amount.to_le_bytes());payload[2..4].copy_from_slice(&date.year.to_le_bytes());payload[4]=date.month.wrapping_add(1);payload[6]=date.day;
    add_inline_letter(pool,merchant,day,month,year,town,0x26,payload,enabled)
}

/// Aviso 0x79 para el acreedor.
pub fn add_debt_alert(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,a:u32,b:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..4].copy_from_slice(&a.to_le_bytes());payload[4..].copy_from_slice(&b.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x79,payload,enabled)
}

/// Carta 0x13 por embargo de mercancías, con el importe y resultado originales.
pub fn add_debt_seizure(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,kind:u8,settled:bool,amount:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[2]=kind;payload[3]=u8::from(settled);payload[4..].copy_from_slice(&amount.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x13,payload,enabled)
}

pub fn add_debt_ship_seizure(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,ship:u16,kind:u8,settled:bool,amount:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..2].copy_from_slice(&ship.to_le_bytes());payload[2]=kind;payload[3]=u8::from(settled);payload[4..].copy_from_slice(&amount.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x13,payload,enabled)
}

/// Aviso 0x13 de cobro parcial en efectivo.
pub fn add_debt_partial(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,amount:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[4..].copy_from_slice(&amount.to_le_bytes());add_inline_letter(pool,merchant,day,month,year,town,0x13,payload,enabled)
}

/// Convocatoria 0x78 de una votación de la Hansa.
pub fn add_hanse_vote(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,a:u32,b:u32,enabled:bool)->Result<bool,String>{
    let mut payload=[0u8;8];payload[..4].copy_from_slice(&a.to_le_bytes());payload[4..].copy_from_slice(&b.to_le_bytes());
    add_inline_letter(pool,merchant,day,month,year,town,0x78,payload,enabled)
}

pub fn add_hanse_vote_result(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x0b,payload,enabled)
}

/// Absolución o condena por boicot (categoría 1).
pub fn add_hanse_verdict(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,kind:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    if !matches!(kind,0x4a|0x4b){return Err("tipo de veredicto desconocido".into())}add_inline_letter(pool,merchant,day,month,year,town,kind,payload,enabled)
}

/// Acusación criminal por romper el boicot (categoría 1).
pub fn add_hanse_indictment(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x19,payload,enabled)
}

/// Convocatoria de la Liga para aportar barcos (categoría 1).
pub fn add_hanse_expedition_request(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x48,payload,enabled)
}

/// Informe de conclusión de la expedición (categoría 1).
pub fn add_hanse_expedition_finished(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x5c,payload,enabled)
}

/// Reproche de la Liga por barcos no aportados (categoría 1).
pub fn add_hanse_expedition_penalty(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    add_inline_letter(pool,merchant,day,month,year,town,0x4c,payload,enabled)
}

/// Carta interna 0x72 de pérdidas navales; categoría 4, sin filtro de plazo.
pub fn add_battle_loss(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,opponent:u16,losses:p3_core::ships::finish_report::Losses)->Result<bool,String>{
    let mut payload=[0u8;8];for (i,v) in [opponent,losses.sunk,losses.captured,losses.looted].iter().enumerate(){payload[i*2..i*2+2].copy_from_slice(&v.to_le_bytes())}
    add_inline_letter(pool,merchant,day,month,year,0,0x72,payload,true)
}

/// Entrega el aviso de botín 0x30 con la preferencia personal de categoría 2.
pub fn add_battle_loot(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16],retention:&[u32])->Result<bool,String>{
    if record[4]!=0x30{return Err("tipo de aviso de botín inválido".into())}
    if merchant.i32(8) as u16!=0{return Ok(false)}
    let setting=usize::from(*merchant.bytes.get(0x39).ok_or("comerciante truncado")?);let enabled=*retention.get(setting).ok_or("preferencia personal inválida")?!=0;
    add_inline_letter(pool,merchant,day,month,year,record[5],0x30,record[8..16].try_into().unwrap(),enabled)
}

/// Informe de batalla 0x29: almacena los bytes del texto como bloque variable
/// en S05. El descriptor opaco se reconstruye al cargar; no es una dirección
/// del ejecutable ni se utiliza para acceder a memoria.
pub fn add_battle_report(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,text:&[u8],retention:&[u32])->Result<bool,String>{
    if merchant.i32(8) as u16!=0{return Ok(false)}
    let setting=usize::from(*merchant.bytes.get(0x39).ok_or("comerciante truncado")?);if *retention.get(setting).ok_or("preferencia personal inválida")?==0{return Ok(false)}
    if text.len()>u16::MAX as usize{return Err("informe naval demasiado largo".into())}
    let mut staged=pool.clone();let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let mut free=usize::from(parsed.field_04);let old=usize::from(parsed.slots);
    if free>=old{let new=old.checked_add(200).filter(|&n|n<=u16::MAX as usize).ok_or("crecimiento del pool fuera de rango")?;free=old;staged[4..6].copy_from_slice(&(new as u16).to_le_bytes());for i in old..new{let mut slot=[0u8;16];slot[1]=0x80;slot[2..6].fill(0xff);slot[6..8].copy_from_slice(&((i+1)as u16).to_le_bytes());staged.extend_from_slice(&slot)}}
    let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let range=parsed.letters[free].range.clone();if range.len()!=16||parsed.letters[free].kind!=0xff{return Err("la ranura libre de mensajes no es válida".into())}
    let next=u16::from_le_bytes([staged[range.start+6],staged[range.start+7]]);let head=(merchant.i32(8)as u32>>16)as u16;let mut letter=vec![0u8;16];letter[0]=day;letter[1]=month|0x80;letter[2..4].copy_from_slice(&year.to_le_bytes());letter[4]=0x29;letter[5]=town;letter[6..8].copy_from_slice(&head.to_le_bytes());letter[8..12].copy_from_slice(&1u32.to_le_bytes());letter[12..16].copy_from_slice(&(text.len()as u32).to_le_bytes());letter.extend_from_slice(text);
    staged.splice(range,letter);staged[6..8].copy_from_slice(&next.to_le_bytes());let word=merchant.i32(8)as u32;merchant.set_i32(8,((word&0xffff)|((free as u32)<<16))as i32);*pool=staged;Ok(true)
}

/// Publica juntos el pool y los comerciantes afectados por un informe 0x72.
/// Los destinatarios fuera de la tabla se omiten como `add_message`.
pub fn persist_battle_losses(model:&mut p3_save::model::SaveModel,report:&p3_core::ships::finish_report::Report)->Result<usize,String>{
    let mut staged=model.clone();let date=&staged.world.header;let(day,month,year)=(date[0],date[1],u16::from_le_bytes([date[2],date[3]]));let mut count=0;
    for notice in &report.notices{let index=usize::from(notice.recipient);let Some(record)=staged.world.merchants.get(index)else{continue};let mut merchant=crate::office::PersistedMerchant::from_save_v85(record).ok_or("comerciante inválido")?;
        if add_battle_loss(&mut staged.message_pool,&mut merchant.merchant,day,month,year,notice.opponent,notice.losses)?{count+=1}
        staged.world.merchants[index]=merchant.to_save_v85();
    }
    *model=staged;Ok(count)
}

fn add_inline_letter(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,kind:u8,payload:[u8;8],enabled:bool)->Result<bool,String>{
    if merchant.i32(8) as u16!=0 || !enabled{return Ok(false);}
    let mut staged=pool.clone();
    let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;
    let mut free=parsed.field_04 as usize;let old_slots=usize::from(parsed.slots);
    if free>=old_slots {
        let new_slots=old_slots.checked_add(200).filter(|&n|n<=u16::MAX as usize).ok_or("crecimiento del pool fuera de rango")?;
        free=old_slots;
        staged[4..6].copy_from_slice(&(new_slots as u16).to_le_bytes());
        for index in old_slots..new_slots {
            let mut slot=[0u8;16];slot[1]=0x80;slot[2..6].fill(0xff);slot[6..8].copy_from_slice(&((index+1) as u16).to_le_bytes());
            staged.extend_from_slice(&slot);
        }
    }
    let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;
    let range=parsed.letters[free].range.clone();
    if range.len()!=16 || parsed.letters[free].kind!=0xff{return Err("la ranura libre de mensajes no es válida".into());}
    let next=u16::from_le_bytes([staged[range.start+6],staged[range.start+7]]);let head=merchant.i32(8) as u32 >>16;
    let mut letter=[0u8;16];letter[0]=day;letter[1]=month|0x80;letter[2..4].copy_from_slice(&year.to_le_bytes());letter[4]=kind;letter[5]=town;letter[6..8].copy_from_slice(&(head as u16).to_le_bytes());letter[8..].copy_from_slice(&payload);
    staged[range].copy_from_slice(&letter);staged[6..8].copy_from_slice(&next.to_le_bytes());
    let word=merchant.i32(8) as u32;merchant.set_i32(8,((word&0xffff)|((free as u32)<<16)) as i32);*pool=staged;Ok(true)
}

/// Categorías de carta de 0x006C0198; 0..=0x85.
const LETTER_CATEGORIES: [u8; 0x86] = [
    2, 3, 3, 2, 2, 2, 4, 3, 2, 4, 1, 1, 4, 4, 4, 4,
    4, 3, 2, 1, 1, 2, 2, 2, 2, 1, 2, 2, 4, 2, 2, 2,
    2, 2, 2, 4, 2, 2, 2, 3, 2, 2, 2, 2, 1, 1, 1, 1,
    2, 1, 1, 2, 2, 4, 1, 1, 4, 4, 4, 1, 4, 1, 2, 3,
    4, 1, 1, 2, 4, 4, 2, 1, 1, 1, 1, 1, 1, 2, 2, 2,
    2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 2, 1, 4,
    2, 2, 2, 3, 1, 1, 3, 3, 3, 3, 2, 1, 1, 1, 1, 1,
    1, 4, 4, 2, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 1, 2,
    1, 2, 1, 2, 1, 1,
];

/// `0x004D6780`: purga las cartas vencidas de un comerciante y las devuelve
/// a la lista libre del depósito S05. Los bloques externos se eliminan junto
/// con su carta. Ante una cadena o fecha inválida no cambia el estado.
pub fn purge_expired(pool:&mut Vec<u8>,merchant:&mut Merchant,tick:u32)->Result<usize,String>{
    const MONTHS:[u32;13]=[0,31,59,90,120,151,181,212,243,273,304,334,365];
    const RETENTION:[u32;6]=[u32::MAX,0x2800,0x1400,0xa00,0x500,0];
    if merchant.bytes.len()<0x3b{return Err("comerciante truncado".into())}
    let parsed=MessagePool::parse(pool).map_err(|e|format!("pool: {e:?}"))?;
    let count=usize::from(parsed.slots);let mut free=parsed.field_04;let mut head=merchant.u16(0xa);
    let mut letters=(0..count).map(|i|parsed.letter(i).to_vec()).collect::<Vec<_>>();
    let mut seen=vec![false;count];let mut previous=None;let mut at=head;let mut removed=0usize;
    while usize::from(at)<count {
        let index=usize::from(at);if seen[index]{return Err("cadena de cartas cíclica".into())}seen[index]=true;
        let letter=&letters[index];let kind=usize::from(letter[4]);let category=*LETTER_CATEGORIES.get(kind).ok_or("tipo de carta inválido")?;
        let month=usize::from(letter[1]&0x7f);let offset=*MONTHS.get(month).ok_or("mes de carta inválido")?;
        let year=u32::from(u16::from_le_bytes([letter[2],letter[3]]));
        let date=year.wrapping_mul(365).wrapping_add(offset).wrapping_add(u32::from(letter[0])).wrapping_sub(1).wrapping_shl(8);
        let age=if tick>date{tick-date}else{0};
        let retention=match category {
            1..=3=>{
                if kind==0x46 {u32::MAX}
                else if kind==0x11 && category==3 {0x700}
                else {let setting=usize::from(merchant.bytes[0x38+usize::from(category)-1]);*RETENTION.get(setting).ok_or("preferencia de cartas inválida")?}
            },
            4=>match kind {
                0x0f|0x10=>0x700,0x72=>0x1c00,
                0x71=>{let d=letter.get(12..16).ok_or("descriptor de carta truncado")?;let deadline=u32::from_le_bytes(d.try_into().unwrap());if tick<deadline{u32::MAX}else{0}},
                _=>u32::MAX,
            },
            _=>0,
        };
        let next=u16::from_le_bytes([letter[6],letter[7]]);
        if age>retention {
            if let Some(prev)=previous {let prior:&mut Vec<u8>=&mut letters[prev];prior[6..8].copy_from_slice(&next.to_le_bytes());}else{head=next}
            let mut blank=vec![0u8;16];blank[2..4].fill(0xff);blank[4]=0xff;blank[6..8].copy_from_slice(&free.to_le_bytes());letters[index]=blank;free=at;removed+=1;
        }else{previous=Some(index)}
        at=next;
    }
    let mut output=pool[..p3_save::tail::MESSAGE_HEADER_LEN].to_vec();output[6..8].copy_from_slice(&free.to_le_bytes());for letter in letters{output.extend_from_slice(&letter)}
    merchant.set_u16(0xa,head);*pool=output;Ok(removed)
}

/// Aviso 0x75 por mercancías perdidas al arder un almacén (categoría 2).
pub fn add_warehouse_fire(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8])->Result<bool,String>{
 let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
 add_inline_letter(pool,merchant,day,month,year,town,0x75,payload,setting!=5)
}

/// Aviso 0x73 por mercancías robadas de un almacén (categoría 2).
pub fn add_warehouse_theft(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,payload:[u8;8])->Result<bool,String>{
 let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}
 add_inline_letter(pool,merchant,day,month,year,town,0x73,payload,setting!=5)
}

/// Carta 0x7A por el nacimiento de un hijo o hija (categoría 2).
pub fn add_candidate_birth(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,town:u8,look:u8)->Result<bool,String>{
 let setting=*merchant.bytes.get(0x39).ok_or("preferencia de cartas ausente")?;if setting>5{return Err("preferencia de cartas inválida".into())}let mut payload=[0u8;8];payload[..2].copy_from_slice(&u16::from(look).to_le_bytes());add_inline_letter(pool,merchant,day,month,year,town,0x7a,payload,setting!=5)
}

/// Inserta una carta de guion `0x3C..0x40/0x71` en S05 como `add_message` (0x004D6530).
/// Cubre destinatario directo, todos los comerciantes humanos (-1) y cadena
/// global (-2), sin avisos de interfaz. Un error no modifica depósito ni mercaderes.
pub fn add_script_letter(pool:&mut Vec<u8>,merchants:&mut [Merchant],merchant_count:u16,day:u8,month:u8,year:u16,letter:&p3_core::mission_letter::ScriptLetter)->Result<usize,String>{
    if !matches!(letter.kind,0x3c..=0x40|0x71)||u32::from_le_bytes(letter.descriptor[..4].try_into().unwrap()) as usize!=letter.text.len(){return Err("carta de guion incompatible".into())}
    let count=usize::from(merchant_count);if merchants.len()<count||merchants[..count].iter().any(|m|m.bytes.len()<12){return Err("comerciantes de carta incompletos".into())}
    let recipient=letter.recipient as i32;
    let targets:Vec<Option<usize>>=match recipient {
        -2=>vec![None],
        -1=>(0..count).filter(|&i|merchants[i].u16(8)==0).map(Some).collect(),
        n if n>=0 && (n as usize)<count=>if merchants[n as usize].u16(8)==0{vec![Some(n as usize)]}else{Vec::new()},
        n if n>=0=>Vec::new(),
        _=>return Err("destinatario de carta de guion no conectado".into()),
    };
    let mut staged=pool.clone();let mut people=merchants.to_vec();let mut delivered=0;
    for target in &targets {
        if let Some(index)=target {
            let mut record=[0u8;16];record[4]=letter.kind;record[5]=letter.town;
            delivered+=usize::from(post_to_merchant(&mut staged,&mut people[*index],day,month,year,&record,Payload::Sized{object:letter.descriptor,block:&letter.text},&RETENTION)?);
            continue;
        }
        delivered+=1;
        let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let mut free=usize::from(parsed.field_04);let old=usize::from(parsed.slots);
        if free>=old {let new=old.checked_add(200).filter(|&n|n<=u16::MAX as usize).ok_or("crecimiento del pool fuera de rango")?;free=old;staged[4..6].copy_from_slice(&(new as u16).to_le_bytes());for i in old..new{let mut slot=[0u8;16];slot[1]=0x80;slot[2..6].fill(0xff);slot[6..8].copy_from_slice(&((i+1) as u16).to_le_bytes());staged.extend_from_slice(&slot)}}
        let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let range=parsed.letters[free].range.clone();if range.len()!=16||parsed.letters[free].kind!=0xff{return Err("ranura libre de mensajes inválida".into())}let next=u16::from_le_bytes(staged[range.start+6..range.start+8].try_into().unwrap());let head=target.map(|i|people[i].u16(10)).unwrap_or(parsed.field_08);
        let mut record=Vec::with_capacity(24+letter.text.len());record.extend_from_slice(&[day,month|0x80]);record.extend_from_slice(&year.to_le_bytes());record.extend_from_slice(&[letter.kind,letter.town]);record.extend_from_slice(&head.to_le_bytes());record.extend_from_slice(&letter.descriptor);record.extend_from_slice(&letter.text);staged.splice(range,record);staged[6..8].copy_from_slice(&next.to_le_bytes());if let Some(i)=target{people[*i].set_u16(10,free as u16)}else{staged[8..10].copy_from_slice(&(free as u16).to_le_bytes())}
    }
    pool.clone_from(&staged);merchants.clone_from_slice(&people);Ok(delivered)
}

/// Destinatario -3 de `0x004D6530`: recibe los dueños en el orden de la
/// cadena urbana. Conserva duplicados; filtra humanos y preferencias en
/// `add_script_letter`. El llamante valida la cadena antes de pasarla.
pub fn add_script_letter_town(pool:&mut Vec<u8>,merchants:&mut [Merchant],merchant_count:u16,day:u8,month:u8,year:u16,letter:&p3_core::mission_letter::ScriptLetter,owners:&[usize])->Result<usize,String>{
    if letter.recipient!=u32::MAX-2{return Err("destinatario urbano inválido".into())}
    let mut staged=pool.clone();let mut people=merchants.to_vec();let mut delivered=0;
    for &owner in owners {if owner>=usize::from(merchant_count){return Err("dueño de oficina de carta ausente".into())}let mut item=letter.clone();item.recipient=owner as u32;delivered+=add_script_letter(&mut staged,&mut people,merchant_count,day,month,year,&item)?;}
    pool.clone_from(&staged);merchants.clone_from_slice(&people);Ok(delivered)
}

/// Clase de entrega de cada tipo de carta (`0x006C0198`, tipos `0..0x86`): 0 no se entrega, 1/2/3 dependen de la preferencia
/// `+0x38/+0x39/+0x3A` del comerciante y 4 se entrega siempre.
const CLASS_OF_KIND: [u8; 0x86] = [2,3,3,2,2,2,4,3,2,4,1,1,4,4,4,4,4,3,2,1,1,2,2,2,2,1,2,2,4,2,2,2,2,2,2,4,2,2,2,3,2,2,2,2,1,1,1,1,2,1,1,2,2,4,1,1,4,4,4,1,4,1,2,3,4,1,1,2,4,4,2,1,1,1,1,1,1,2,2,2,2,2,2,4,4,4,4,4,4,4,3,2,1,2,1,4,2,2,2,3,1,1,3,3,3,3,2,1,1,1,1,1,1,4,4,2,2,2,1,1,1,2,2,1,2,1,1,2,1,2,1,2,1,1];
/// Retención por preferencia (`0x006C0220`); el valor 0 significa «no guardar».
pub const RETENTION: [u32; 6] = [u32::MAX, 0x2800, 0x1400, 0xa00, 0x500, 0];

/// Carga útil de una carta que el original copia fuera de los 16 bytes (`0x0051AB20`).
#[derive(Clone, Copy, Debug)]
pub enum Payload<'a> {
    /// Tipos sin bloque.
    None,
    /// Tipos `0x29/0x47/0x63/0x83`: texto de `len` bytes tras la cabecera (`+8` = 1, `+0xC` = longitud).
    Text(&'a [u8]),
    /// Tipos `0x3C..=0x40/0x71`: objeto de 16 bytes cuya primera palabra es el tamaño del bloque, y el bloque.
    Sized { object: [u8; 16], block: &'a [u8] },
}

/// Entrega a un comerciante el cuerpo de `0x004D6680` (sin la notificación de interfaz `0x004D7B10`): filtra por humano y por la
/// clase del tipo, y la guarda en la lista del comerciante.
pub fn post_to_merchant(pool:&mut Vec<u8>,merchant:&mut Merchant,day:u8,month:u8,year:u16,record:&[u8;16],payload:Payload<'_>,retention:&[u32])->Result<bool,String>{
    let kind=record[4];if kind>=0x86{return Ok(false)}
    if merchant.i32(8) as u16!=0{return Ok(false)}
    let setting=|at:usize|->Result<usize,String>{Ok(usize::from(*merchant.bytes.get(at).ok_or("comerciante truncado")?))};
    let keep=|index:usize|->Result<bool,String>{Ok(*retention.get(index).ok_or("preferencia personal inválida")?>0)};
    let deliver=match CLASS_OF_KIND[usize::from(kind)]{
        0=>false,
        1=>keep(setting(0x38)?)?,
        2=>kind==0x46||keep(setting(0x39)?)?,
        3=>kind==0x11||keep(setting(0x3a)?)?,
        4=>true,
        _=>false,
    };
    if !deliver{return Ok(false)}
    let mut letter=vec![0u8;16];letter[..16].copy_from_slice(record);letter[0]=day;letter[1]=month|0x80;letter[2..4].copy_from_slice(&year.to_le_bytes());
    match (kind,payload){
        (0x29|0x47|0x63|0x83,Payload::Text(text))=>{if text.len()>u32::MAX as usize{return Err("texto demasiado largo".into())}letter[8..12].copy_from_slice(&1u32.to_le_bytes());letter[12..16].copy_from_slice(&(text.len() as u32).to_le_bytes());letter.extend_from_slice(text)}
        (0x3c..=0x40|0x71,Payload::Sized{object,block})=>{if u32::from_le_bytes([object[0],object[1],object[2],object[3]]) as usize!=block.len(){return Err("el objeto no describe el bloque".into())}letter.truncate(8);letter.extend_from_slice(&object);letter.extend_from_slice(block)}
        (0x29|0x47|0x63|0x83|0x3c..=0x40|0x71,_)=>return Err("carta con bloque sin carga útil".into()),
        (_,Payload::None)=>{}
        _=>return Err("carga útil en un tipo sin bloque".into()),
    }
    let mut staged=pool.clone();let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let mut free=usize::from(parsed.field_04);let old=usize::from(parsed.slots);
    if free>=old{let new=old.checked_add(200).filter(|&n|n<=u16::MAX as usize).ok_or("crecimiento del pool fuera de rango")?;free=old;staged[4..6].copy_from_slice(&(new as u16).to_le_bytes());for i in old..new{let mut slot=[0u8;16];slot[1]=0x80;slot[2..6].fill(0xff);slot[6..8].copy_from_slice(&((i+1)as u16).to_le_bytes());staged.extend_from_slice(&slot)}}
    let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let range=parsed.letters[free].range.clone();if range.len()!=16||parsed.letters[free].kind!=0xff{return Err("la ranura libre de mensajes no es válida".into())}
    let next=u16::from_le_bytes([staged[range.start+6],staged[range.start+7]]);let head=(merchant.i32(8)as u32>>16)as u16;letter[6..8].copy_from_slice(&head.to_le_bytes());
    staged.splice(range,letter);staged[6..8].copy_from_slice(&next.to_le_bytes());let word=merchant.i32(8)as u32;merchant.set_i32(8,((word&0xffff)|((free as u32)<<16))as i32);*pool=staged;Ok(true)
}

/// `0x004D6530` sin la notificación de interfaz: `code` ≥ 0 es un comerciante, −1 todos, −2 carta global. El código −3
/// (carta a la ciudad) no está portado.
pub fn post_message(pool:&mut Vec<u8>,merchants:&mut [Merchant],merchant_count:u16,day:u8,month:u8,year:u16,code:i32,record:&[u8;16],payload:Payload<'_>,retention:&[u32])->Result<usize,String>{
    if record[4]>=0x86||code>=i32::from(merchant_count){return Ok(0)}
    match code{
        -2=>{if payload_is_some(&payload){return Err("carta global con bloque no portada".into())}add_global_any(pool,day,month,year,record)?;Ok(1)}
        -1=>{let mut n=0;let mut working=pool.clone();for m in merchants.iter_mut().take(usize::from(merchant_count)){if post_to_merchant(&mut working,m,day,month,year,record,payload,retention)?{n+=1}}*pool=working;Ok(n)}
        -3=>Err("código -3 de 0x004D6530 no portado".into()),
        c if c>=0=>{let m=merchants.get_mut(c as usize).ok_or("destinatario ausente")?;Ok(usize::from(post_to_merchant(pool,m,day,month,year,record,payload,retention)?))}
        _=>Ok(0),
    }
}
fn payload_is_some(p:&Payload<'_>)->bool{!matches!(p,Payload::None)}
/// Carta global del original: ranura libre al principio de la lista global (`+8`), sin lista por comerciante.
fn add_global_any(pool:&mut Vec<u8>,day:u8,month:u8,year:u16,record:&[u8;16])->Result<(),String>{
    let mut staged=pool.clone();let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let mut free=usize::from(parsed.field_04);let count=usize::from(parsed.slots);
    if free>=count{let new=count.checked_add(200).filter(|&n|n<=u16::MAX as usize).ok_or("crecimiento del pool fuera de rango")?;free=count;staged[4..6].copy_from_slice(&(new as u16).to_le_bytes());for i in count..new{let mut slot=[0u8;16];slot[1]=0x80;slot[2..6].fill(0xff);slot[6..8].copy_from_slice(&((i+1)as u16).to_le_bytes());staged.extend_from_slice(&slot)}}
    let parsed=MessagePool::parse(&staged).map_err(|e|format!("pool: {e:?}"))?;let range=parsed.letters[free].range.clone();if range.len()!=16||parsed.letters[free].kind!=0xff{return Err("ranura libre de mensajes inválida".into())}
    let next=u16::from_le_bytes([staged[range.start+6],staged[range.start+7]]);let head=parsed.field_08;let mut letter=*record;letter[0]=day;letter[1]=month|0x80;letter[2..4].copy_from_slice(&year.to_le_bytes());letter[6..8].copy_from_slice(&head.to_le_bytes());staged[range].copy_from_slice(&letter);staged[6..8].copy_from_slice(&next.to_le_bytes());staged[8..10].copy_from_slice(&(free as u16).to_le_bytes());*pool=staged;Ok(())
}
