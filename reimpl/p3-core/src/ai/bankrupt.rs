//! Reinicio de un comerciante de IA por bancarrota (`0x004F5D80`, C06).

use crate::ai::misc::NameHost;
use crate::ai::reputation::Merchant;

/// Misión de la tabla `0x006DD73C`: siguiente `+4`, tipo `+6`, bytes `+0x10`, `+0x11` y `+0xC`.
#[derive(Clone, Copy, Default)]
pub struct Mission {
    pub next: u16,
    pub kind: u16,
    pub b10: u8,
    pub b11: u8,
    pub bc: u8,
}

/// Globales que lee el reinicio.
pub struct BankruptcyEnv<'a> {
    /// Índice del comerciante (cociente `0x005303E0`) y dirección del objeto (divisor si no hay comerciantes).
    pub index: u32,
    pub this_addr: u32,
    /// `0x006DE4B4`, `0x006DE4A4`, `0x006DDB70`, `0x006DDB74`, `0x006DD888`.
    pub date: u32,
    pub word_4a4: u32,
    pub name_count: u32,
    pub name2_count: u32,
    pub dword_888: u32,
    /// `0x006DE4AA` (comerciantes), `0x006DE4A2` (año), `0x006DD744` (cursor) y `0x006DD748` (misiones).
    pub merchant_count: u16,
    pub year: u16,
    pub cursor: u32,
    pub mission_count: u16,
    /// Byte `+0x1D` de los demás comerciantes (el propio se lee de su estructura).
    pub last_names: &'a [u8],
}

/// Ganchos: avisos `0x54E1F0`/`0x54E170`, misiones `0x4D8DB0`, cola de mensajes `0x4D6780` y reserva `0x64F7B9`.
pub trait BankruptcyHost: NameHost {
    fn on_event(&mut self, tag: &'static str, arg: u32);
}

impl Merchant {
    /// `0x004F5D80`: solo para IA (`+8 != 0`). Quita al comerciante de las misiones de tipo 1, deja 800.000 de límite (`+0x53C`),
    /// borra reputación, mapa y libro, le da 1.000.000 de oro, nueva identidad y nombre, y libera su puesto en la ciudad.
    /// El apellido elegido evita los ya usados; si no hay comerciantes el divisor es la dirección del objeto (conservado).
    pub fn bankruptcy_reset(
        &mut self,
        env: &BankruptcyEnv,
        missions: &mut [Mission],
        town_flags: &mut [u8],
        host: &mut dyn BankruptcyHost,
    ) {
        self.try_bankruptcy_reset(env, missions, town_flags, host).expect("quiebra fuera de dominio");
    }

    /// Como [`Merchant::bankruptcy_reset`], pero devuelve `Err` donde el original divide por cero (`0x004F5F57`):
    /// el byte `+0x1D` del último comerciante (el propio ya reiniciado si es el último) vale 0. El comerciante
    /// queda entonces a medio reiniciar; el llamador debe descartarlo.
    pub fn try_bankruptcy_reset(
        &mut self,
        env: &BankruptcyEnv,
        missions: &mut [Mission],
        town_flags: &mut [u8],
        host: &mut dyn BankruptcyHost,
    ) -> Result<(), &'static str> {
        let idx = env.index;
        if self.u16(8) == 0 {
            return Ok(());
        }
        host.on_event("m1", idx);
        let mut cur = env.cursor;
        while (cur as u16) < env.mission_count {
            let i = cur & 0xFFFF;
            host.on_event("mission", i);
            let m = &mut missions[(i as usize).min(missions.len() - 1)];
            if m.kind == 1 {
                if m.b10 as u16 == idx as u16 {
                    m.b10 = 0xFF;
                } else if m.b11 as u16 == idx as u16 {
                    m.b11 = 0xFF;
                    m.bc = 0xFF;
                }
            }
            cur = m.next as u32;
        }
        self.set_i32(0x53C, 0xC3500);
        self.set_i32(0x540, 0);
        self.set_u16(0x544, 0);
        self.set_i32(0x608, 0);
        let day = env.date as u8;
        self.bytes[0x546] = day;
        self.bytes[0x547] = day.wrapping_mul(7);
        self.bytes[0x548..0x5A8].fill(0);
        self.bytes[0x5A8..0x608].fill(0);
        self.reset_reputation_and_ledger();
        self.set_i32(0x464, 0x4000_0000);
        self.bytes[0x4D8..0x538].fill(0);
        self.set_i32(0, 1_000_000);
        self.init_identity_and_birth(env.date, env.date & 3, 0, 1, env.year);
        self.bytes[0x26] = 0;
        self.bytes[0x38..0x40].fill(5);
        host.on_event("purge", idx & 0xFFFF);
        self.bytes[0x1C] = ((env.word_4a4 & 0xFFFF) % (env.name_count & 0xFFFF)) as u8;
        let n2 = (env.name2_count & 0xFFFF) as usize;
        host.on_event("alloc", n2 as u32);
        let mut buf = vec![0u8; 0x1000];
        let mut divisor = env.this_addr;
        if env.merchant_count > 0 {
            for k in 0..env.merchant_count as usize {
                let b = if k as u32 == idx {
                    self.bytes[0x1D]
                } else {
                    env.last_names[k]
                } as usize;
                divisor = b as u32;
                if n2 > b {
                    buf[b] = 1;
                }
            }
        }
        if divisor == 0 {
            return Err("apellido nuevo módulo cero (el original divide por cero en 0x004F5F57)");
        }
        self.bytes[0x1D] = (((env.dword_888 & 0xFFFF) + (env.word_4a4 & 0xFFFF)) % divisor) as u8;
        while buf[self.bytes[0x1D] as usize] != 0 {
            self.bytes[0x1D] = self.bytes[0x1D].wrapping_add(1);
            if self.bytes[0x1D] as u32 > divisor {
                self.bytes[0x1D] = 0;
            }
        }
        host.on_event("m2", idx);
        let flag = &mut town_flags[self.bytes[0x19] as usize];
        if *flag as u16 == idx as u16 {
            *flag = 0xFF;
        }
        Ok(())
    }
}
