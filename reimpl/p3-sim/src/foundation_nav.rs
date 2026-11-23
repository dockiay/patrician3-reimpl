//! Datos del planificador de puertos usados por fundación de ciudades.
use p3_core::route::{finder::RoutePlanner,planner::{load_nav_data,NavData,NavFiles,TownSlot}};

#[derive(Clone,Debug)]
pub struct FoundationNavigation { nav:NavData, slots:Vec<TownSlot> }

impl FoundationNavigation {
    /// Crea el planificador para rutas entre puntos sin modificar la tabla de puertos.
    pub fn point_planner(&self)->RoutePlanner {
        RoutePlanner::new(self.nav.clone(),self.slots.clone(),vec![],0,vec![],vec![])
    }
    /// Carga recursos por precedencia de archivos; `None` indica que ningún
    /// archivo de navegación está presente. Datos parciales se rechazan.
    pub fn from_archives(archives:&[&[u8]],town_data:&[u8])->Result<Option<Self>,String>{
        let parsed=archives.iter().map(|b|p3_assets::cpr::Archive::parse(b).map_err(|e|format!("CPR: {e}"))).collect::<Result<Vec<_>,_>>()?;
        let mut parts=Vec::new();for name in ["navdata/nav_matrix.dat","navdata/nav_vec.dat","navdata/matrix_int.dat"]{let mut data=None;for a in parsed.iter().rev(){if let Some(e)=a.find(name.as_bytes()){data=Some(a.read(e).ok_or_else(||format!("recurso no disponible: {name}"))?.to_vec());break}}parts.push(data)}
        if parts.iter().all(Option::is_none){return Ok(None)}
        let files=parts.into_iter().collect::<Option<Vec<_>>>().ok_or("faltan archivos de navegación")?;
        if files[0].len()<4||files[1].len()<4{return Err("cabecera de navegación truncada".into())}
        let width=i16::from_le_bytes([files[0][0],files[0][1]]);let height=i16::from_le_bytes([files[0][2],files[0][3]]);let count=i16::from_le_bytes([files[1][0],files[1][1]]);
        if width<=0||height<=0||count<=0{return Err("dimensiones de navegación inválidas".into())}
        let n=count as usize;if files[0].len()!=4+width as usize*height as usize||files[1].len()!=4+n*4||files[2].len()!=n.checked_mul(n).and_then(|v|v.checked_mul(6)).ok_or("matriz de navegación excesiva")?{return Err("tamaño de navegación inválido".into())}
        struct Files(Vec<Vec<u8>>);impl NavFiles for Files{fn open(&mut self,i:usize)->Option<Vec<u8>>{self.0.get(i).cloned()}}
        let nav=load_nav_data(&mut Files(files),0xdd0).ok_or("navegación inválida")?;
        if nav.matrix.iter().flatten().any(|&(_,next)|next<0||next>=count){return Err("nodo de navegación inválido".into())}
        for target in 0..n{let mut state=vec![0u8;n];state[target]=2;for start in 0..n{let mut at=start;let mut path=Vec::new();while state[at]==0{state[at]=1;path.push(at);at=nav.matrix[at][target].1 as usize;}if state[at]==1{return Err("matriz de navegación cíclica".into())}for i in path{state[i]=2;}}}
        let slots=crate::merchant_daily::parse_town_slots(town_data)?;Ok(Some(Self{nav,slots}))
    }

    pub fn build_planner(&self,order:&[u8],max_rings:u32)->Result<RoutePlanner,String>{
        if order.len()>40||order.iter().any(|&i|usize::from(i)>=self.slots.len())||max_rings==0{return Err("orden de puertos inválido".into())}
        let mut planner=RoutePlanner::new(self.nav.clone(),self.slots.clone(),order.to_vec(),order.len() as u32,vec![],vec![]);
        if !planner.build_town_route_table(max_rings){return Err("no se encontraron puntos de mar para los puertos".into())}Ok(planner)
    }
}
