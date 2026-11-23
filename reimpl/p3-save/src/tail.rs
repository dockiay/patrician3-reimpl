//! Bloques pequeÃƒÂ±os del `.pat` (S04): `message_pool`, `route_stop_pool`, `scheduled_tasks`, `sea_battles` y `name_bank`.
//!
//! Cada lector consume exactamente su bloque (si sobra o falta algo, error) y entrega rangos de los registros, sin copiar. Lo que el original
//! no muestra con datos (los objetos de batalla virtuales de `sea_battles`) se rechaza con
//! `Unsupported`. Las listas de bajas de tareas `0x34` se conservan como bytes opacos. Fuentes: `research/systems/borde-motor/save-fields/*.tsv` y los escritores
//! `0x004D62B0`/`0x0051A8D0` (mensajes), `0x004D4B50` (paradas), `0x004D80F0` (tareas), `0x00611960` (batallas) y `0x00511C60` (nombres).

use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub enum TailError {
    Truncated,
    Marker(u32),
    Trailing { used: usize, len: usize },
    Unsupported(&'static str),
    Terminator { group: usize },
}

fn u16_at(d: &[u8], o: usize) -> Result<u16, TailError> {
    d.get(o..o + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or(TailError::Truncated)
}

fn u32_at(d: &[u8], o: usize) -> Result<u32, TailError> {
    d.get(o..o + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]])).ok_or(TailError::Truncated)
}

fn finish(used: usize, data: &[u8]) -> Result<(), TailError> {
    if used == data.len() {
        Ok(())
    } else if used > data.len() {
        Err(TailError::Truncated)
    } else {
        Err(TailError::Trailing { used, len: data.len() })
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------------------
// message_pool

pub const MESSAGE_MARKER: u32 = 0xAFFE_0005;
pub const MESSAGE_HEADER_LEN: usize = 12;
pub const LETTER_HEAD: usize = 8;
/// Tipos de carta (`+4`) con 8 bytes en lÃƒÂ­nea y un bloque de `u32[+0xC]` bytes.
pub const LETTER_INLINE_BLOB: [u8; 4] = [0x29, 0x47, 0x63, 0x83];

/// CÃƒÂ³mo termina una carta tras sus 8 bytes de cabecera.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LetterBody {
    /// Solo 8 bytes mÃƒÂ¡s en lÃƒÂ­nea (`+8..+0x10`), sin bloque.
    Inline,
    /// 8 bytes en lÃƒÂ­nea cuyo `u32` en `+0xC` es el tamaÃƒÂ±o del bloque que sigue.
    InlineBlob,
    /// 16 bytes (de un objeto aparte) cuya primera palabra es el tamaÃƒÂ±o del bloque que sigue: tipos `0x3C..=0x40` y `0x71`.
    SizedBlob,
}

pub fn letter_body(kind: u8) -> LetterBody {
    if LETTER_INLINE_BLOB.contains(&kind) {
        LetterBody::InlineBlob
    } else if (0x3C..=0x40).contains(&kind) || kind == 0x71 {
        LetterBody::SizedBlob
    } else {
        LetterBody::Inline
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Letter {
    /// Bytes de la carta en el bloque (cabecera, cuerpo y bloque).
    pub range: Range<usize>,
    pub kind: u8,
    pub body: LetterBody,
}

#[derive(Debug)]
pub struct MessagePool<'a> {
    data: &'a [u8],
    /// `+4`: nÃ‚Âº de ranuras (`obj+6`; 200 en el corpus).
    pub slots: u16,
    /// `+6` (`obj+4`) y `+8` (`obj+8`): contadores del gestor; su significado no estÃƒÂ¡ probado.
    pub field_04: u16,
    pub field_08: u16,
    pub letters: Vec<Letter>,
}

impl<'a> MessagePool<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, TailError> {
        let marker = u32_at(data, 0)?;
        if marker != MESSAGE_MARKER {
            return Err(TailError::Marker(marker));
        }
        let slots = u16_at(data, 4)?;
        let (field_04, field_08) = (u16_at(data, 6)?, u16_at(data, 8)?);
        let mut pos = MESSAGE_HEADER_LEN;
        let mut letters = Vec::with_capacity(usize::from(slots));
        for _ in 0..slots {
            let start = pos;
            let kind = *data.get(pos + 4).ok_or(TailError::Truncated)?;
            let body = letter_body(kind);
            pos += match body {
                LetterBody::Inline => LETTER_HEAD + 8,
                LetterBody::InlineBlob => LETTER_HEAD + 8 + u32_at(data, pos + 12)? as usize,
                LetterBody::SizedBlob => LETTER_HEAD + 16 + u32_at(data, pos + 8)? as usize,
            };
            if pos > data.len() {
                return Err(TailError::Truncated);
            }
            letters.push(Letter { range: start..pos, kind, body });
        }
        finish(pos, data)?;
        Ok(MessagePool { data, slots, field_04, field_08, letters })
    }

    pub fn letter(&self, i: usize) -> &'a [u8] {
        &self.data[self.letters[i].range.clone()]
    }

    /// Cartas distintas de las ranuras vacÃƒÂ­as (`kind == 0xFF`).
    pub fn occupied(&self) -> impl Iterator<Item = (usize, &Letter)> {
        self.letters.iter().enumerate().filter(|(_, l)| l.kind != 0xFF)
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------------------
// route_stop_pool

pub const ROUTE_STOP_LEN: usize = 220;

#[derive(Debug)]
pub struct RouteStopPool {
    /// `obj+0`, `obj+2`.
    pub field_00: u16,
    pub count: u16,
    pub stops: Range<usize>,
}

impl RouteStopPool {
    pub fn parse(data: &[u8]) -> Result<Self, TailError> {
        let (field_00, count) = (u16_at(data, 0)?, u16_at(data, 2)?);
        let end = 4 + ROUTE_STOP_LEN * usize::from(count);
        finish(end, data)?;
        Ok(RouteStopPool { field_00, count, stops: 4..end })
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------------------
// scheduled_tasks

pub const TASK_LEN: usize = 24;
/// Tipo de tarea (`+6`) que aÃƒÂ±ade `u16[+0x12]` palabras de 4 bytes.
pub const TASK_WITH_LIST: u16 = 0x1B;
/// Tipo con lista de bajas opcional desde la fase 7 (`0x004F39E0`).
pub const TASK_WITH_OBJECT: u16 = 0x34;

/// Actualiza la tabla fija sin reinterpretar los bloques colgantes. Los
/// registros que poseen datos adicionales deben seguir conservando su carga.
pub fn update_scheduled_task_table(data: &[u8], scheduler: &p3_core::scheduler::Scheduler) -> Result<Vec<u8>, String> {
    if data.len()<6 {return Err("cabecera de tareas truncada".into())}
    let old_count=usize::from(u16::from_le_bytes([data[4],data[5]]));
    let tail=6+TASK_LEN*old_count;
    if data.len()<tail || scheduler.records.len()!=usize::from(scheduler.capacity) {return Err("tabla de tareas invÃƒÂ¡lida".into())}
    for (i,old) in data[6..tail].chunks_exact(TASK_LEN).enumerate() {
        let kind=u16::from_le_bytes([old[6],old[7]]);
        if kind==TASK_WITH_LIST {
            let Some(new)=scheduler.records.get(i) else{return Err("tarea con datos externos eliminada".into())};
            if new.kind!=kind || new.payload!=old[8..24] {return Err("tarea con datos externos modificada".into())}
        } else if kind==TASK_WITH_OBJECT && old[0x15]>6 && old[8..12]!=[0;4] {
            let Some(new)=scheduler.records.get(i) else{return Err("lista de pÃƒÂ©rdidas eliminada".into())};
            if new.kind!=kind || new.u8_at(0x15)<=6 || new.payload[..4]!=old[8..12] {return Err("lista de pÃƒÂ©rdidas modificada".into())}
        }
    }
    for (i,new) in scheduler.records.iter().enumerate(){if new.kind==TASK_WITH_OBJECT && new.u8_at(0x15)>6 && new.u32_at(8)!=0 {let old=data.get(6+i*TASK_LEN..6+(i+1)*TASK_LEN);if old.is_none_or(|r|u16::from_le_bytes([r[6],r[7]])!=TASK_WITH_OBJECT||r[0x15]<=6||r[8..12]!=new.payload[..4]){return Err("lista de pÃƒÂ©rdidas nueva no serializada".into())}}}
    let mut out=Vec::with_capacity(6+TASK_LEN*scheduler.records.len()+data.len()-tail);
    out.extend_from_slice(&scheduler.head.to_le_bytes());out.extend_from_slice(&scheduler.free_head.to_le_bytes());out.extend_from_slice(&scheduler.capacity.to_le_bytes());
    for r in &scheduler.records {out.extend_from_slice(&r.due.to_le_bytes());out.extend_from_slice(&r.next.to_le_bytes());out.extend_from_slice(&r.kind.to_le_bytes());out.extend_from_slice(&r.payload);}
    out.extend_from_slice(&data[tail..]);Ok(out)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub range: Range<usize>,
    pub kind: u16,
    /// Bytes de lista tras los 24 del registro (vacÃƒÂ­o salvo en las tareas `0x1B`).
    pub extra: Range<usize>,
}

#[derive(Debug)]
pub struct ScheduledTasks<'a> {
    data: &'a [u8],
    /// `obj+8`, `obj+0xA`; `count` es `obj+0xC`.
    pub field_08: u16,
    pub field_0a: u16,
    pub count: u16,
    pub tasks: Vec<Task>,
}

impl<'a> ScheduledTasks<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, TailError> {
        let (field_08, field_0a, count) = (u16_at(data, 0)?, u16_at(data, 2)?, u16_at(data, 4)?);
        let table = 6..6 + TASK_LEN * usize::from(count);
        if table.end > data.len() {
            return Err(TailError::Truncated);
        }
        let mut tasks = Vec::with_capacity(usize::from(count));
        let mut extra_pos = table.end;
        for i in 0..usize::from(count) {
            let at = table.start + TASK_LEN * i;
            let kind = u16_at(data, at + 6)?;
            let n = match kind {
                TASK_WITH_LIST => 4 * usize::from(u16_at(data, at + 0x12)?),
                TASK_WITH_OBJECT if data[at+0x15]>6 && data[at+8..at+12]!=[0;4] => {
                    let mut cursor=extra_pos.checked_add(4).ok_or(TailError::Truncated)?;
                    loop {let marker=*data.get(cursor).ok_or(TailError::Truncated)?;cursor+=1;if marker==0{break}if marker!=0x7f{return Err(TailError::Unsupported("marca de lista de pÃƒÂ©rdidas"))}cursor=cursor.checked_add(4).ok_or(TailError::Truncated)?;if cursor>data.len(){return Err(TailError::Truncated)}}cursor-extra_pos
                },
                _ => 0,
            };
            tasks.push(Task { range: at..at + TASK_LEN, kind, extra: extra_pos..extra_pos + n });
            extra_pos += n;
        }
        finish(extra_pos, data)?;
        Ok(ScheduledTasks { data, field_08, field_0a, count, tasks })
    }

    pub fn task(&self, i: usize) -> &'a [u8] {
        &self.data[self.tasks[i].range.clone()]
    }

    pub fn extra(&self, i: usize) -> &'a [u8] {
        &self.data[self.tasks[i].extra.clone()]
    }
}

/// Carga de una tarea de guion y su tabla de variables; no interpreta punteros del guardado.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ScriptSnapshot { pub payload:[u8;16], pub variables:Vec<u32> }

pub fn script_snapshots(data:&[u8])->Result<Vec<Option<ScriptSnapshot>>,String>{
    let parsed=ScheduledTasks::parse(data).map_err(|e|format!("tareas: {e:?}"))?;
    Ok(parsed.tasks.iter().enumerate().map(|(i,t)|if t.kind==TASK_WITH_LIST{
        let mut payload=[0u8;16];payload.copy_from_slice(&parsed.task(i)[8..24]);
        Some(ScriptSnapshot{payload,variables:parsed.extra(i).chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().expect("palabra completa"))).collect()})
    }else{None}).collect())
}

/// Reescribe variables y cursor de guiones existentes, preservando listas ajenas.
/// El estado previo debe ser el inicial, el último guardado por la sesión o el
/// estado actual; así los cambios de otro sistema se rechazan antes de escribir.
/// Nodo de la lista de bajas de la caza hanseática (`0x004F3980`): `+4` clase, `+5` marca, `+6` texto.
pub use p3_core::ships::return_convoy::HuntLoss;

/// `0x004F39E0` sobre la cadena que empieza en la cabeza (`tarea+8`): 4 bytes por nodo
/// `{clase, marca, sar(+6 bajo, 8), +6 bajo}`, `0x7F` antes de cada hijo y `0` al final.
/// La palabra alta de `+6` no se guarda. Una lista vacía no tiene volcado (`tarea+8 = 0`).
pub fn encode_hunt_losses(nodes:&[HuntLoss])->Vec<u8>{
    let mut out=Vec::with_capacity(5*nodes.len());
    for (i,n) in nodes.iter().enumerate(){if i>0{out.push(0x7f)}let low=n.text as u8;out.extend_from_slice(&[n.class,n.mark,((low as i8)>>7) as u8,low])}
    if !nodes.is_empty(){out.push(0)}out
}

/// `0x004F3A40`: rehace `+6` como `b * 0x101` con el byte de signo y sigue mientras la marca sea
/// `0x7F`; el byte que la corta se consume sea cual sea. Devuelve los nodos y los bytes usados.
pub fn decode_hunt_losses(data:&[u8])->Result<(Vec<HuntLoss>,usize),TailError>{
    let mut nodes=Vec::new();let mut at=0usize;
    loop{
        let r=data.get(at..at+4).ok_or(TailError::Truncated)?;nodes.push(HuntLoss{class:r[0],mark:r[1],text:u16::from(r[2])*0x101});at+=4;
        let marker=*data.get(at).ok_or(TailError::Truncated)?;at+=1;if marker!=0x7f{return Ok((nodes,at))}
    }
}

/// Listas de bajas de las tareas `0x34` (fase `>= 7`, `+8` no nulo) por ranura, tal como las deja el lector.
pub fn hunt_loss_lists(data:&[u8])->Result<Vec<Option<Vec<HuntLoss>>>,String>{
    let parsed=ScheduledTasks::parse(data).map_err(|e|format!("tareas: {e:?}"))?;
    parsed.tasks.iter().enumerate().map(|(i,t)|if t.kind==TASK_WITH_OBJECT&&!t.extra.is_empty(){decode_hunt_losses(parsed.extra(i)).map(|(n,_)|Some(n)).map_err(|e|format!("lista de bajas {i}: {e:?}"))}else{Ok(None)}).collect()
}

pub fn update_scheduled_tasks_with_scripts(data:&[u8],scheduler:&p3_core::scheduler::Scheduler,initial:&[Option<ScriptSnapshot>],last:Option<&[Option<ScriptSnapshot>]>,variables:&[Option<Vec<u32>>])->Result<Vec<u8>,String>{
    update_scheduled_tasks_with_lists(data,scheduler,initial,last,variables,None)
}

/// Como [`update_scheduled_tasks_with_scripts`]; con `losses` las listas de bajas de las tareas
/// `0x34` se escriben desde el modelo vivo (una entrada no vacía por cada tarea con `+8` no nulo
/// en fase `>= 7`) en lugar de exigir que no cambien.
pub fn update_scheduled_tasks_with_lists(data:&[u8],scheduler:&p3_core::scheduler::Scheduler,initial:&[Option<ScriptSnapshot>],last:Option<&[Option<ScriptSnapshot>]>,variables:&[Option<Vec<u32>>],losses:Option<&[Option<Vec<HuntLoss>>]>)->Result<Vec<u8>,String>{
    let parsed=ScheduledTasks::parse(data).map_err(|e|format!("tareas: {e:?}"))?;let old=script_snapshots(data)?;
    if scheduler.records.len()!=usize::from(scheduler.capacity){return Err("tabla de tareas inválida".into())}
    for (i,record) in scheduler.records.iter().enumerate(){
        let previous=old.get(i).and_then(Option::as_ref);
        if record.kind==TASK_WITH_LIST {
            let saved=previous.ok_or("guion nuevo no serializado")?;let vars=variables.get(i).and_then(Option::as_ref).ok_or("variables de guion ausentes")?;
            let count=usize::from(u16::from_le_bytes([record.u8_at(0x12),record.u8_at(0x13)]));if vars.len()!=count{return Err("recuento de variables de guion incompatible".into())}
            let mut identity=saved.payload;identity[12..14].copy_from_slice(&record.payload[12..14]);if identity!=record.payload{return Err("identidad de guion modificada".into())}
            let current=ScriptSnapshot{payload:record.payload,variables:vars.clone()};
            let known=initial.get(i).and_then(Option::as_ref)==Some(saved)||last.and_then(|v|v.get(i)).and_then(Option::as_ref)==Some(saved)||&current==saved;
            if !known{return Err(format!("variables o cursor del guion {i} modificados por otro sistema"))}
        }else if let Some(saved)=previous {
            let known=initial.get(i).and_then(Option::as_ref)==Some(saved)||last.and_then(|v|v.get(i)).and_then(Option::as_ref)==Some(saved);
            if !known{return Err(format!("guion {i} eliminado con estado ajeno"))}
            // `None` indica que la sesión terminó el guion. Su ranura puede
            // haberse reutilizado antes del siguiente guardado.
            // Un registro libre que conserva el puntero del guion (liberado sin ponerlo a cero, `remove_by_kind`) no se
            // puede reproducir; si la ranura se reutilizó y volvió a liberarse, `+0xC` es dato de la otra tarea.
            if !matches!(variables.get(i),Some(None))||(record.kind==p3_core::scheduler::FREE&&record.u32_at(0xc)!=0&&record.payload[4..8]==saved.payload[4..8]){return Err(format!("eliminación de guion {i} incompatible (tipo {:#x}, +0xC {:#x}, variables {})",record.kind,record.u32_at(0xc),if variables.get(i).is_some_and(Option::is_some){"vivas"}else{"ausentes"}))}
        }
        let has_object=record.kind==TASK_WITH_OBJECT&&record.u8_at(0x15)>6&&record.u32_at(8)!=0;
        if let Some(lists)=losses{
            let live=lists.get(i).and_then(Option::as_ref).is_some_and(|l|!l.is_empty());
            if live!=has_object{return Err(format!("lista de bajas {i} incoherente con su tarea"))}
            continue;
        }
        let prior=parsed.tasks.get(i);
        let had_object=prior.is_some_and(|t|t.kind==TASK_WITH_OBJECT&&!t.extra.is_empty());
        if had_object!=has_object{return Err("lista de pérdidas modificada".into())}
        if has_object {let old=parsed.task(i);if record.payload[..4]!=old[8..12]{return Err("lista de pérdidas modificada".into())}}
    }
    if old.len()>scheduler.records.len()&&old[scheduler.records.len()..].iter().any(Option::is_some){return Err("guion fuera del planificador".into())}
    if parsed.tasks.len()>scheduler.records.len()&&parsed.tasks[scheduler.records.len()..].iter().any(|t|!t.extra.is_empty()){return Err("datos de tarea fuera del planificador".into())}
    let mut out=Vec::new();out.extend_from_slice(&scheduler.head.to_le_bytes());out.extend_from_slice(&scheduler.free_head.to_le_bytes());out.extend_from_slice(&scheduler.capacity.to_le_bytes());
    for r in &scheduler.records{out.extend_from_slice(&r.due.to_le_bytes());out.extend_from_slice(&r.next.to_le_bytes());out.extend_from_slice(&r.kind.to_le_bytes());out.extend_from_slice(&r.payload)}
    for (i,r) in scheduler.records.iter().enumerate(){if r.kind==TASK_WITH_LIST{for value in variables[i].as_ref().expect("validado"){out.extend_from_slice(&value.to_le_bytes())}}else if r.kind==TASK_WITH_OBJECT&&r.u8_at(0x15)>6&&r.u32_at(8)!=0{match losses{Some(lists)=>out.extend_from_slice(&encode_hunt_losses(lists[i].as_deref().expect("validado"))),None=>out.extend_from_slice(parsed.extra(i))}}}
    ScheduledTasks::parse(&out).map_err(|e|format!("tareas actualizadas: {e:?}"))?;Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------------------
// sea_battles

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeaBattle {
    pub index: u32,
    /// Pares de `u32` (8 bytes cada uno).
    pub entries: Range<usize>,
}

#[derive(Debug)]
pub struct SeaBattles<'a> {
    data: &'a [u8],
    /// Primer byte del bloque (siempre 0 en el corpus).
    pub first: u8,
    pub battles: Vec<SeaBattle>,
}

impl<'a> SeaBattles<'a> {
    /// El escritor emite antes de la lista, por cada uno de los 255 punteros no nulos, un byte de clase y un volcado virtual; con todos nulos
    /// (el caso del corpus) no emite nada. El lector detecta un puntero no nulo por una longitud que no encaja y devuelve `Unsupported`.
    pub fn parse(data: &'a [u8]) -> Result<Self, TailError> {
        let first = *data.first().ok_or(TailError::Truncated)?;
        let n = u32_at(data, 1)? as usize;
        let mut pos = 5;
        let mut battles = Vec::new();
        for _ in 0..n {
            let index = u32_at(data, pos)?;
            let len = u32_at(data, pos + 4)? as usize;
            let entries = pos + 8..pos + 8 + 8 * len;
            if entries.end > data.len() {
                return Err(TailError::Truncated);
            }
            pos = entries.end;
            battles.push(SeaBattle { index, entries });
        }
        if pos != data.len() {
            return Err(TailError::Unsupported("sea_battles con objetos de batalla virtuales (punteros no nulos)"));
        }
        Ok(SeaBattles { data, first, battles })
    }

    pub fn entries(&self, i: usize) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.data[self.battles[i].entries.clone()]
            .chunks_exact(8)
            .map(|c| (u32::from_le_bytes(c[..4].try_into().unwrap()), u32::from_le_bytes(c[4..].try_into().unwrap())))
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------------------
// name_bank

pub const NAME_FILLER: usize = 80;
pub const NAME_GROUPS: usize = 3;
pub const NAMES_PER_GROUP: usize = 20;
pub const NAME_END: u8 = b'#';

#[derive(Debug)]
pub struct NameBank<'a> {
    data: &'a [u8],
    /// Tres grupos de veinte nombres terminados en cero (rangos sin el cero).
    pub groups: [Vec<Range<usize>>; NAME_GROUPS],
    /// Nombres dinÃƒÂ¡micos de barco: cuenta y bloque (`u16` de tamaÃƒÂ±o y sus bytes).
    pub dynamic_count: u16,
    pub dynamic_blob: Range<usize>,
}

impl<'a> NameBank<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, TailError> {
        if data.len() < NAME_FILLER {
            return Err(TailError::Truncated);
        }
        let mut pos = NAME_FILLER;
        let mut groups: [Vec<Range<usize>>; NAME_GROUPS] = Default::default();
        for (g, group) in groups.iter_mut().enumerate() {
            for _ in 0..NAMES_PER_GROUP {
                let rel = data.get(pos..).ok_or(TailError::Truncated)?.iter().position(|&b| b == 0).ok_or(TailError::Truncated)?;
                group.push(pos..pos + rel);
                pos += rel + 1;
            }
            if data.get(pos) != Some(&NAME_END) {
                return Err(TailError::Terminator { group: g });
            }
            pos += 1;
        }
        let dynamic_count = u16_at(data, pos)?;
        let size = usize::from(u16_at(data, pos + 2)?);
        let dynamic_blob = pos + 4..pos + 4 + size;
        finish(dynamic_blob.end, data)?;
        Ok(NameBank { data, groups, dynamic_count, dynamic_blob })
    }

    pub fn name(&self, group: usize, i: usize) -> &'a [u8] {
        &self.data[self.groups[group][i].clone()]
    }

    pub fn dynamic_blob(&self) -> &'a [u8] {
        &self.data[self.dynamic_blob.clone()]
    }
}
/// Escribe el bloque actual `sea_battles` (0x00611950) con batallas base.
/// Las entradas se recorren por ranura 0..FE, como el gestor original. La
/// variante de ataque a ciudad requiere su propio serializador y no se acepta.
pub fn serialize_basic_sea_battles(battles:&[Option<p3_core::ships::battle::BattleSaveInput<'_>>],pairs:&[Vec<(u32,u32)>])->Result<Vec<u8>,TailError>{
 if battles.len()>255{return Err(TailError::Unsupported("mÃƒÂ¡s de 255 ranuras de combate"))}let count=battles.iter().filter(|b|b.is_some()).count();let mut out=vec![count as u8];for battle in battles.iter().flatten(){out.push(0);let payload=p3_core::ships::battle::serialize_basic_battle(battle).map_err(TailError::Unsupported)?;out.extend_from_slice(&payload)}let nonempty=pairs.iter().filter(|v|!v.is_empty()).count();let nonempty=u32::try_from(nonempty).map_err(|_|TailError::Unsupported("demasiadas listas"))?;out.extend_from_slice(&nonempty.to_le_bytes());for (i,entries) in pairs.iter().enumerate(){if entries.is_empty(){continue}let index=u32::try_from(i).map_err(|_|TailError::Unsupported("ÃƒÂ­ndice de lista excesivo"))?;let len=u32::try_from(entries.len()).map_err(|_|TailError::Unsupported("lista excesiva"))?;out.extend_from_slice(&index.to_le_bytes());out.extend_from_slice(&len.to_le_bytes());for &(a,b) in entries{out.extend_from_slice(&a.to_le_bytes());out.extend_from_slice(&b.to_le_bytes())}}Ok(out)
}
/// Encuadra el formato actual (versiÃƒÂ³n >= 0x5D) de batallas base y pares.
/// Conserva los cuerpos como rangos; no reconstruye objetos, RNG ni colisiones.
#[derive(Debug)]
pub struct CurrentSeaBattles<'a>{data:&'a [u8],pub objects:Vec<Range<usize>>,pub pairs:Vec<SeaBattle>}
fn current_battle_take(data:&[u8],pos:&mut usize,len:usize)->Result<Range<usize>,TailError>{let end=pos.checked_add(len).ok_or(TailError::Truncated)?;if end>data.len(){return Err(TailError::Truncated)}let result=*pos..end;*pos=end;Ok(result)}
impl<'a> CurrentSeaBattles<'a>{
 pub fn parse(data:&'a [u8])->Result<Self,TailError>{let count=usize::from(*data.first().ok_or(TailError::Truncated)?);let mut pos=1usize;let mut objects=Vec::with_capacity(count);for _ in 0..count{let kind=*data.get(pos).ok_or(TailError::Truncated)?;pos+=1;if kind!=0{return Err(TailError::Unsupported("ataque a ciudad o variante de combate"))}let start=pos;let header=current_battle_take(data,&mut pos,0x27)?;let h=&data[header];let participants=usize::from(h[0]);let n0=usize::from(u16::from_le_bytes([h[1],h[2]]));let n1=usize::from(u16::from_le_bytes([h[3],h[4]]));let n=n0.checked_add(n1).filter(|&v|v<=u16::MAX as usize).ok_or(TailError::Unsupported("conteo local excesivo"))?;let heads=[u16::from_le_bytes([h[5],h[6]]),u16::from_le_bytes([h[7],h[8]])];let participant_bytes=participants.checked_mul(4).and_then(|v|v.checked_add(participants.min(2)*4)).ok_or(TailError::Truncated)?;current_battle_take(data,&mut pos,participant_bytes)?;let words=usize::from(u16_at(data,pos)?);pos+=2;let bitmap_range=current_battle_take(data,&mut pos,words.checked_mul(4).ok_or(TailError::Truncated)?)?;let active=data[bitmap_range].chunks_exact(4).map(|b|u32::from_le_bytes([b[0],b[1],b[2],b[3]]).count_ones() as usize).sum::<usize>();let links=current_battle_take(data,&mut pos,n.checked_mul(2).ok_or(TailError::Truncated)?)?;let nexts=data[links].chunks_exact(2).map(|b|u16::from_le_bytes([b[0],b[1]])).collect::<Vec<_>>();current_battle_take(data,&mut pos,n.checked_mul(2).ok_or(TailError::Truncated)?)?;current_battle_take(data,&mut pos,0x400)?;let mut visits=0usize;for mut id in heads{let mut seen=vec![false;n];while id!=u16::MAX{let i=usize::from(id);if i>=n||seen[i]{return Err(TailError::Unsupported("lista local invÃƒÂ¡lida"))}seen[i]=true;visits=visits.checked_add(1).ok_or(TailError::Truncated)?;id=nexts[i]}}let payload=visits.checked_mul(47).and_then(|v|active.checked_mul(31).and_then(|a|v.checked_add(a))).ok_or(TailError::Truncated)?;current_battle_take(data,&mut pos,payload)?;objects.push(start..pos)}let pair_count=u32_at(data,pos)? as usize;pos=pos.checked_add(4).ok_or(TailError::Truncated)?;let mut pairs=Vec::new();for _ in 0..pair_count{let index=u32_at(data,pos)?;let len=u32_at(data,pos.checked_add(4).ok_or(TailError::Truncated)?)? as usize;pos=pos.checked_add(8).ok_or(TailError::Truncated)?;let entries=current_battle_take(data,&mut pos,len.checked_mul(8).ok_or(TailError::Truncated)?)?;pairs.push(SeaBattle{index,entries})}if pos!=data.len(){return Err(TailError::Trailing{used:pos,len:data.len()})}Ok(Self{data,objects,pairs})
 }
 pub fn body(&self,index:usize)->Option<&'a [u8]>{self.objects.get(index).map(|r|&self.data[r.clone()])}
 pub fn entries(&self,index:usize)->Option<impl Iterator<Item=(u32,u32)>+'a>{self.pairs.get(index).map(|pair|self.data[pair.entries.clone()].chunks_exact(8).map(|c|(u32::from_le_bytes(c[..4].try_into().expect("entrada completa")),u32::from_le_bytes(c[4..].try_into().expect("entrada completa")))))}
}
/// Barco local leÃƒÂ­do del bloque de batalla; solo estÃƒÂ¡n poblados los campos
/// persistidos y los derivados de su carga, no es todavÃƒÂ­a un objeto vivo.
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct SavedLocalBattleShip{pub index:u16,pub side:u8,pub image:Vec<u8>}
impl CurrentSeaBattles<'_>{
 /// Lee los registros actuales de ambos bandos en el orden de las listas.
 /// El llamante debe reconstruir por separado colisiones y estado de batalla.
 pub fn local_ships(&self,index:usize,battle_address:u32)->Result<Vec<SavedLocalBattleShip>,TailError>{
  let body=self.body(index).ok_or(TailError::Unsupported("batalla inexistente"))?;if body.len()<0x27{return Err(TailError::Truncated)}let participants=usize::from(body[0]);let n=usize::from(u16_at(body,1)?)+usize::from(u16_at(body,3)?);let heads=[u16_at(body,5)?,u16_at(body,7)?];let mut pos=0x27usize;current_battle_take(body,&mut pos,participants*4+participants.min(2)*4)?;let words=usize::from(u16_at(body,pos)?);pos+=2;current_battle_take(body,&mut pos,words*4)?;let links=current_battle_take(body,&mut pos,n*2)?;let nexts=body[links].chunks_exact(2).map(|v|u16::from_le_bytes([v[0],v[1]])).collect::<Vec<_>>();current_battle_take(body,&mut pos,n*2)?;current_battle_take(body,&mut pos,0x400)?;let mut result=Vec::new();let mut visited=vec![false;n];for (side,mut id) in heads.into_iter().enumerate(){while id!=u16::MAX{let i=usize::from(id);if i>=n||visited[i]{return Err(TailError::Unsupported("lista local invÃƒÂ¡lida o compartida"))}visited[i]=true;let record=current_battle_take(body,&mut pos,47)?;let mut image=vec![0;0x164];p3_core::ships::battle::deserialize_local_ship(&mut image,&body[record],0x5d,battle_address).map_err(TailError::Unsupported)?;result.push(SavedLocalBattleShip{index:id,side:side as u8,image});id=nexts[i]}}Ok(result)
 }
}

/// Estado persistido de una batalla bÃƒÂ¡sica inicial de dos barcos. No contiene
/// las tablas espaciales ni los recursos transitorios necesarios para simularla.
#[derive(Debug, Clone)]
pub struct SavedInitialCommercialBattle {
    pub object: Vec<u8>,
    pub next: Vec<u16>,
    pub previous: Vec<u16>,
    pub local_ships: Vec<Vec<u8>>,
    pub bitmap: Vec<u32>,
    pub projectiles: Vec<Vec<u8>>,
}
impl SavedInitialCommercialBattle {
    pub fn save_input(&self) -> p3_core::ships::battle::BattleSaveInput<'_> {
        p3_core::ships::battle::BattleSaveInput {
            object: &self.object, nexts: &self.next, prevs: &self.previous,
            bitmap: &self.bitmap, local_ships: &self.local_ships, projectiles: &self.projectiles,
        }
    }
}
impl CurrentSeaBattles<'_> {
    /// Reconstruye los campos guardados de una batalla inicial de dos barcos.
    /// No pretende reconstruir los ÃƒÂ­ndices de colisiÃƒÂ³n ni habilitar su tick.
    pub fn initial_commercial_state(&self, index: usize, battle_address: u32, slot: u8)
        -> Result<SavedInitialCommercialBattle, TailError>
    {
        if slot == u8::MAX { return Err(TailError::Unsupported("ranura invÃƒÂ¡lida")); }
        let body = self.body(index).ok_or(TailError::Unsupported("batalla inexistente"))?;
        if body.len() < 0x27 { return Err(TailError::Truncated); }
        if body[0] != 2 || u16_at(body, 1)? != 1 || u16_at(body, 3)? != 1 {
            return Err(TailError::Unsupported("se requieren dos bandos de un barco"));
        }
        let mut object = vec![0; 0x1e98];
        let mut manager = vec![0; 0x405];
        p3_core::ships::battle::construct_basic_battle(&mut object, &mut manager, battle_address, u32::from(slot))
            .ok_or(TailError::Unsupported("constructor invÃƒÂ¡lido"))?;
        let mut pos = 0;
        for (at, len) in [(0x558,1),(0x658,2),(0x65a,2),(0x10,2),(0x65c,2),
            (0x65e,2),(0x660,2),(0xad4,2),(0xad6,2),(0xad8,2),(0xada,2),
            (0xadc,4),(0x12e8,1),(0x130c,2),(0x130e,1),(0x5c,1),(0x670,1),
            (0x671,1),(0x1e90,1),(0x1e91,1),(0xaca,1),(0xacb,1),(0x1e94,2),(0xad2,1)] {
            let range = current_battle_take(body, &mut pos, len)?;
            object[at..at+len].copy_from_slice(&body[range]);
        }
        if u16::from_le_bytes([object[0x10],object[0x11]]) != 0 || u16::from_le_bytes([object[0x65c],object[0x65d]]) != 1 {
            return Err(TailError::Unsupported("orden de bandos no admitido"));
        }
        object[0x14..0x16].copy_from_slice(&2u16.to_le_bytes());
        for i in 0..2 {
            let r = current_battle_take(body, &mut pos, 8)?;
            let p = &body[r];
            object[0x35a+i*2..0x35c+i*2].copy_from_slice(&p[..2]);
            object[0x5d+i] = p[2]; object[0x559+i] = p[3];
            object[0xa84+i*4..0xa88+i*4].copy_from_slice(&p[4..8]);
        }
        if u16::from_le_bytes([object[0x35a],object[0x35b]]) >= 0xfe || u16::from_le_bytes([object[0x35c],object[0x35d]]) >= 0xfe {
            return Err(TailError::Unsupported("participante especial no admitido"));
        }
        let words = u16_at(body, pos)?; pos += 2;
        if words != 0 { return Err(TailError::Unsupported("proyectiles no admitidos")); }
        let read_links = |pos: &mut usize| -> Result<Vec<u16>, TailError> {
            let r = current_battle_take(body, pos, 4)?;
            Ok(body[r].chunks_exact(2).map(|v| u16::from_le_bytes([v[0],v[1]])).collect())
        };
        let next = read_links(&mut pos)?; let previous = read_links(&mut pos)?;
        if next != [u16::MAX; 2] || previous != [u16::MAX; 2] {
            return Err(TailError::Unsupported("enlaces locales incompatibles"));
        }
        let range = current_battle_take(body, &mut pos, 0x400)?;
        object[0x672..0xa72].copy_from_slice(&body[range]);
        let records = self.local_ships(index, battle_address)?;
        if records.len() != 2 || records[0].side != 0 || records[1].side != 1 || records[0].index == records[1].index {
            return Err(TailError::Unsupported("listas locales invÃƒÂ¡lidas"));
        }
        let mut local_ships = vec![Vec::new(); 2];
        for record in records { local_ships[usize::from(record.index)] = record.image; }
        let result = SavedInitialCommercialBattle { object, next, previous, local_ships, bitmap:vec![], projectiles:vec![] };
        let encoded = p3_core::ships::battle::serialize_basic_battle(&result.save_input())
            .map_err(TailError::Unsupported)?;
        if encoded != body { return Err(TailError::Unsupported("reconstrucciÃƒÂ³n no reversible")); }
        Ok(result)
    }
}

/// Estado reconstruido por la carga inicial. Las tablas espaciales son Rust;
/// los campos de puntero de la imagen no contienen direcciones utilizables.
#[derive(Debug, Clone)]
pub struct RebuiltInitialCommercialBattle {
    pub state: SavedInitialCommercialBattle,
    pub collisions: p3_core::ships::battle::BattleCollisionPool,
    pub spatial_0: Vec<u16>,
    pub spatial_1: Vec<u16>,
}
/// Índices espaciales de las dos tablas guardadas (`0x00604D87..`).
fn rebuild_battle_obstacles(object:&[u8],pool:&mut p3_core::ships::battle::BattleCollisionPool)->Result<(),TailError>{
    for (base,kind,expanded) in [(0x672usize,3u8,5u8),(0x872,2,4)]{
        for id in 0..64usize{
            let at=base+id*8;let get=|offset|i32::from(i16::from_le_bytes([object[at+offset],object[at+offset+1]]));
            let a=get(0);if a == -1 {continue}let b=get(2);let c=get(4);let d=get(6);let center=a*2+(b&1);
            let x0=(center-c+1)*17;let y0=(b-c-d+1)*17;let x1=(center+d+1)*17;let y1=(b+1)*17;
            let slot=pool.add_growing(kind,id as u8,x0 as u16,y0 as u16,x1 as u16,y1 as u16).map_err(TailError::Unsupported)?;
            let i=usize::from(slot)*2;let bounds=[pool.x[i].wrapping_sub(200),pool.y[i].wrapping_sub(200),pool.x[i+1].wrapping_add(200),pool.y[i+1].wrapping_add(200)];
            pool.add_growing(expanded,id as u8,bounds[0],bounds[1],bounds[2],bounds[3]).map_err(TailError::Unsupported)?;
        }
    }
    Ok(())
}
impl SavedInitialCommercialBattle {
    /// Reconstruye las asignaciones espaciales y constructores de barcos del
    /// cargador actual para dos barcos, sin proyectiles ni combate avanzado.
    pub fn rebuild(mut self, world_ships: &[u8], battle_address: u32)
        -> Result<RebuiltInitialCommercialBattle, TailError>
    {
        use p3_core::ships::battle::{BattleCollisionPool,construct_local_ship,deserialize_local_ship,serialize_local_ship};
        const SHIP: usize = 0x180;
        let default_ship = world_ships.get(..SHIP).ok_or(TailError::Unsupported("flota truncada"))?;
        let mut pool = BattleCollisionPool::with_capacity(0).map_err(TailError::Unsupported)?;
        rebuild_battle_obstacles(&self.object,&mut pool)?;
        let mut spatial_0 = vec![0;2]; let mut spatial_1 = vec![0;2];
        for i in 0..2 {
            let record = serialize_local_ship(&self.local_ships[i]).map_err(TailError::Unsupported)?;
            let index = u32::from_le_bytes(self.local_ships[i][4..8].try_into().expect("barco completo")) as usize;
            let ship = world_ships.get(index.checked_mul(SHIP).ok_or(TailError::Truncated)?..index.checked_add(1).and_then(|n| n.checked_mul(SHIP)).ok_or(TailError::Truncated)?)
                .ok_or(TailError::Unsupported("barco mundial inexistente"))?;
            let mut local = vec![0;0x164];
            construct_local_ship(&mut local,default_ship,0,battle_address,i as u16,0).map_err(TailError::Unsupported)?;
            deserialize_local_ship(&mut local,&record,0x5d,battle_address).map_err(TailError::Unsupported)?;
            let mut actual = vec![0;0x164];
            construct_local_ship(&mut actual,ship,index as u32,battle_address,i as u16,0).map_err(TailError::Unsupported)?;
            local[0x13c] = actual[0x13c];
            self.local_ships[i] = local;
            spatial_0[i] = pool.add_growing(0,i as u8,0,0,0,0).map_err(TailError::Unsupported)?;
            spatial_1[i] = pool.add_growing(6,i as u8,0,0,0,0).map_err(TailError::Unsupported)?;
        }
        let o=&mut self.object;
        o[0xac8]=u8::from(o[0x5d]!=0&&o[0x5e]!=0); o[0x1e92]=1; o[0xab0]=1; o[0xab1]=1;
        o[0x15c..0x160].fill(0);
        if o[0x5c] != 0 { o[0x15c] = 1; }
        for (at,v) in [(0x20,pool.free_head),(0x34,pool.x_head),(0x44,pool.y_head),
            (0x54,pool.high_water),(0x56,pool.kinds.len() as u16),(0x58,(pool.kinds.len()*2) as u16),
            (0x5a,pool.matrices[0].len() as u16)] { o[at..at+2].copy_from_slice(&v.to_le_bytes()); }
        Ok(RebuiltInitialCommercialBattle { state:self,collisions:pool,spatial_0,spatial_1 })
    }
}

impl CurrentSeaBattles<'_> {
    /// Reconstruye los datos serializados de una batalla inicial de dos bandos
    /// con uno o más barcos por bando, incluidos participantes 0xFE/0xFF.
    /// Los proyectiles se leen, pero el índice espacial se reconstruye aparte.
    pub fn initial_convoy_state(&self,index:usize,battle_address:u32,slot:u8)->Result<SavedInitialCommercialBattle,TailError>{
        if slot==u8::MAX{return Err(TailError::Unsupported("ranura inválida"))}
        let body=self.body(index).ok_or(TailError::Unsupported("batalla inexistente"))?;
        if body.len()<0x27{return Err(TailError::Truncated)}
        let n0=usize::from(u16_at(body,1)?);let n1=usize::from(u16_at(body,3)?);
        let n=n0.checked_add(n1).filter(|&n|n<=u16::MAX as usize).ok_or(TailError::Unsupported("conteo local excesivo"))?;
        if body[0]!=2||n==0{return Err(TailError::Unsupported("participantes o conteo local inválidos"))}
        let mut object=vec![0;0x1e98];let mut manager=vec![0;0x405];
        p3_core::ships::battle::construct_basic_battle(&mut object,&mut manager,battle_address,u32::from(slot)).ok_or(TailError::Unsupported("constructor inválido"))?;
        let mut pos=0;
        for (at,len) in [(0x558,1),(0x658,2),(0x65a,2),(0x10,2),(0x65c,2),(0x65e,2),(0x660,2),(0xad4,2),(0xad6,2),(0xad8,2),(0xada,2),(0xadc,4),(0x12e8,1),(0x130c,2),(0x130e,1),(0x5c,1),(0x670,1),(0x671,1),(0x1e90,1),(0x1e91,1),(0xaca,1),(0xacb,1),(0x1e94,2),(0xad2,1)]{
            let r=current_battle_take(body,&mut pos,len)?;object[at..at+len].copy_from_slice(&body[r]);
        }
        object[0x14..0x16].copy_from_slice(&(n as u16).to_le_bytes());
        for i in 0..2{let r=current_battle_take(body,&mut pos,8)?;let p=&body[r];object[0x35a+i*2..0x35c+i*2].copy_from_slice(&p[..2]);object[0x5d+i]=p[2];object[0x559+i]=p[3];object[0xa84+i*4..0xa88+i*4].copy_from_slice(&p[4..8]);}
        let words=usize::from(u16_at(body,pos)?);pos+=2;if words>2047{return Err(TailError::Unsupported("bitmap de proyectiles excesivo"))}
        let bits=current_battle_take(body,&mut pos,words*4)?;let bitmap=body[bits].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().expect("palabra completa"))).collect::<Vec<_>>();
        let slots=words*32;object[0xacc..0xace].copy_from_slice(&(slots as u16).to_le_bytes());object[0xace..0xad0].copy_from_slice(&(words as u16).to_le_bytes());
        let read_links=|pos:&mut usize|->Result<Vec<u16>,TailError>{let r=current_battle_take(body,pos,n*2)?;Ok(body[r].chunks_exact(2).map(|v|u16::from_le_bytes([v[0],v[1]])).collect())};
        let next=read_links(&mut pos)?;let previous=read_links(&mut pos)?;
        let mut seen=vec![false;n];for (head,expected) in [(u16::from_le_bytes([object[0x10],object[0x11]]),n0),(u16::from_le_bytes([object[0x65c],object[0x65d]]),n1)]{let mut id=head;let mut prior=u16::MAX;let mut count=0usize;while id!=u16::MAX{let i=usize::from(id);if i>=n||seen[i]||previous[i]!=prior{return Err(TailError::Unsupported("enlaces locales incompatibles"))}seen[i]=true;count+=1;prior=id;id=next[i];}if count!=expected{return Err(TailError::Unsupported("conteo de bando incompatible"))}}if seen.iter().any(|x|!*x){return Err(TailError::Unsupported("listas locales incompletas"))}
        let r=current_battle_take(body,&mut pos,0x400)?;object[0x672..0xa72].copy_from_slice(&body[r]);
        let records=self.local_ships(index,battle_address)?;if records.len()!=n{return Err(TailError::Unsupported("listas locales incompletas"))}
        let mut local_ships=vec![Vec::new();n];for record in records{let id=usize::from(record.index);if id>=n||record.image[0x13d]!=record.side||!local_ships[id].is_empty(){return Err(TailError::Unsupported("listas locales inválidas"))}local_ships[id]=record.image;}
        current_battle_take(body,&mut pos,n*47)?;
        let mut projectiles=vec![vec![0;0x38];slots];for id in 0..slots{if bitmap[id/32]&(1<<(id%32))==0{continue}let r=current_battle_take(body,&mut pos,31)?;let p:&mut p3_core::ships::projectile::Projectile=projectiles[id].as_mut_slice().try_into().expect("proyectil completo");p3_core::ships::projectile::deserialize_current(p,&body[r],battle_address,id as u16,0).map_err(TailError::Unsupported)?;}
        let result=SavedInitialCommercialBattle{object,next,previous,local_ships,bitmap,projectiles};
        let encoded=p3_core::ships::battle::serialize_basic_battle(&result.save_input()).map_err(TailError::Unsupported)?;if encoded!=body{return Err(TailError::Unsupported("reconstrucción no reversible"))}Ok(result)
    }
}

impl SavedInitialCommercialBattle {
    /// Reconstruye los barcos y tablas espaciales de una batalla inicial de
    /// convoyes; no ejecuta su simulación ni restaura efectos de interfaz.
    pub fn rebuild_convoy(mut self,world_ships:&[u8],battle_address:u32)->Result<RebuiltInitialCommercialBattle,TailError>{
        use p3_core::ships::battle::{BattleCollisionPool,construct_local_ship,deserialize_local_ship,serialize_local_ship};
        if !self.bitmap.is_empty(){return Err(TailError::Unsupported("usar reconstrucción con proyectiles y RNG"))}
        const SHIP:usize=0x180;
        let n=self.local_ships.len();if n==0||n>256{return Err(TailError::Unsupported("conteo local fuera del índice espacial admitido"))}
        let default_ship=world_ships.get(..SHIP).ok_or(TailError::Unsupported("flota truncada"))?;
        let mut pool=BattleCollisionPool::with_capacity(0).map_err(TailError::Unsupported)?;let mut spatial_0=vec![0;n];let mut spatial_1=vec![0;n];
        rebuild_battle_obstacles(&self.object,&mut pool)?;
        // El cargador original crea los registros y huecos espaciales siguiendo
        // las dos cadenas, que tras una captura no tienen por qué ir por ID.
        let mut order=Vec::with_capacity(n);let mut seen=vec![false;n];for at in [0x10usize,0x65c]{let mut id=u16::from_le_bytes([self.object[at],self.object[at+1]]);while id!=u16::MAX{let i=usize::from(id);if i>=n||seen[i]{return Err(TailError::Unsupported("lista local inválida"))}seen[i]=true;order.push(i);id=*self.next.get(i).ok_or(TailError::Unsupported("enlace local ausente"))?;}}if order.len()!=n{return Err(TailError::Unsupported("listas locales incompletas"))}
        for i in order{
            let record=serialize_local_ship(&self.local_ships[i]).map_err(TailError::Unsupported)?;
            let id=u32::from_le_bytes(self.local_ships[i][4..8].try_into().expect("barco completo")) as usize;
            let start=id.checked_mul(SHIP).ok_or(TailError::Truncated)?;let end=start.checked_add(SHIP).ok_or(TailError::Truncated)?;let ship=world_ships.get(start..end).ok_or(TailError::Unsupported("barco mundial inexistente"))?;
            let mut local=vec![0;0x164];construct_local_ship(&mut local,default_ship,0,battle_address,i as u16,0).map_err(TailError::Unsupported)?;deserialize_local_ship(&mut local,&record,0x5d,battle_address).map_err(TailError::Unsupported)?;
            let mut actual=vec![0;0x164];construct_local_ship(&mut actual,ship,id as u32,battle_address,i as u16,0).map_err(TailError::Unsupported)?;local[0x13c]=actual[0x13c];self.local_ships[i]=local;
            spatial_0[i]=pool.add_growing(0,i as u8,0,0,0,0).map_err(TailError::Unsupported)?;spatial_1[i]=pool.add_growing(6,i as u8,0,0,0,0).map_err(TailError::Unsupported)?;
        }
        let o=&mut self.object;o[0xac8]=u8::from(o[0x5d]!=0&&o[0x5e]!=0);o[0x1e92]=1;o[0xab0]=1;o[0xab1]=if u16::from_le_bytes([o[0x35a],o[0x35b]])==0xfe&&u16::from_le_bytes([o[0x35c],o[0x35d]])==0xff{2}else{1};o[0x15c..0x160].fill(0);if o[0x5c]!=0{o[0x15c]=1}
        for (at,v) in [(0x20,pool.free_head),(0x34,pool.x_head),(0x44,pool.y_head),(0x54,pool.high_water),(0x56,pool.kinds.len() as u16),(0x58,(pool.kinds.len()*2) as u16),(0x5a,pool.matrices[0].len() as u16)]{o[at..at+2].copy_from_slice(&v.to_le_bytes())}
        Ok(RebuiltInitialCommercialBattle{state:self,collisions:pool,spatial_0,spatial_1})
    }

    /// Restaura proyectiles activos después de crear barcos e índice espacial.
    /// El constructor y el lector consumen cada uno una llamada al RNG CRT.
    pub fn rebuild_convoy_with_projectiles<F:FnMut()->u32>(mut self,world_ships:&[u8],battle_address:u32,mut random:F)->Result<RebuiltInitialCommercialBattle,TailError>{
        use p3_core::ships::projectile::{self,InitArgs,ProjectileTable,Spatial};
        struct Pool<'a>(&'a mut p3_core::ships::battle::BattleCollisionPool);
        impl Spatial for Pool<'_>{
            fn bounds(&mut self,_:u16,_:i16,_:i16,_:i16,_:i16){unreachable!("constructor no actualiza límites")}
            fn remove(&mut self,_:u16){unreachable!("constructor no retira objetos")}
            fn add(&mut self,kind:u8,id:u8,x0:i16,y0:i16,x1:i16,y1:i16)->Option<u16>{self.0.add_growing(kind,id,x0 as u16,y0 as u16,x1 as u16,y1 as u16).ok()}
        }
        let bitmap=std::mem::take(&mut self.bitmap);let records=std::mem::take(&mut self.projectiles);
        let mut rebuilt=self.rebuild_convoy(world_ships,battle_address)?;
        let slots=bitmap.len().checked_mul(32).filter(|&n|n<=u16::MAX as usize).ok_or(TailError::Unsupported("bitmap de proyectiles excesivo"))?;
        if records.len()!=slots{return Err(TailError::Unsupported("proyectiles truncados"))}
        let mut table=ProjectileTable::new(slots);
        for id in 0..slots{
            if bitmap[id/32]&(1<<(id%32))==0{continue}
            let saved=&records[id];if saved.len()<0x26{return Err(TailError::Truncated)}
            let mut record=Vec::with_capacity(31);for (at,len) in [(0,2),(2,2),(4,2),(6,2),(8,2),(10,2),(12,4),(16,4),(20,4),(24,1),(28,4),(36,2)]{record.extend_from_slice(&saved[at..at+len])}
            let args=InitArgs{id:id as u16,dest:(1,0),src:(0,0),damage:0,kind:12,shooter:255,tag:8};
            let mut p=projectile::init(&mut table,battle_address,rebuilt.state.object[0xac8]!=0,&mut Pool(&mut rebuilt.collisions),&mut random,args).map_err(TailError::Unsupported)?;
            projectile::deserialize_current(&mut p,&record,battle_address,id as u16,random()).map_err(TailError::Unsupported)?;
            table.slots[id]=Some(p);
        }
        rebuilt.state.bitmap=bitmap;rebuilt.state.projectiles=table.slots.into_iter().map(|p|p.map(|v|v.to_vec()).unwrap_or_else(||vec![0;0x38])).collect();
        let o=&mut rebuilt.state.object;o[0xacc..0xace].copy_from_slice(&(slots as u16).to_le_bytes());o[0xace..0xad0].copy_from_slice(&(rebuilt.state.bitmap.len() as u16).to_le_bytes());
        for (at,v) in [(0x20,rebuilt.collisions.free_head),(0x34,rebuilt.collisions.x_head),(0x44,rebuilt.collisions.y_head),(0x54,rebuilt.collisions.high_water),(0x56,rebuilt.collisions.kinds.len() as u16),(0x58,(rebuilt.collisions.kinds.len()*2) as u16),(0x5a,rebuilt.collisions.matrices[0].len() as u16)]{o[at..at+2].copy_from_slice(&v.to_le_bytes())}
        Ok(rebuilt)
    }
}
