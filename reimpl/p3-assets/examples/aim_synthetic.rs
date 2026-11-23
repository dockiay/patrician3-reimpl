//! Genera `.aim` sintéticos para las ramas de `AIM.dll` que el corpus inglés no ejercita (planos intercalados de `0x21`,
//! profundidades menores que 8 y alfa de profundidad 0, `0x1b`/`0x1c`, bloques sin comprimir, copia literal con cabecera
//! negativa, `0x16` con formato 5) y escribe en `argv[1]` cada archivo y `expected.tsv` con lo que devuelve
//! `aim::decode`. `reimpl/oracle/aim/aim_reencoded.py --synthetic DIR` los pasa por la DLL y compara.
use p3_assets::aim::{self, deflate};
use sha2::{Digest, Sha256};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as u32
    }
    fn bytes(&mut self, n: usize, runs: bool) -> Vec<u8> {
        let mut v = Vec::with_capacity(n);
        while v.len() < n {
            let b = self.next() as u8;
            let k = if runs { 1 + self.next() as usize % 9 } else { 1 };
            for _ in 0..k.min(n - v.len()) {
                v.push(b);
            }
        }
        v
    }
}

fn hex(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

fn file(sub: Vec<u8>) -> Vec<u8> {
    let mut sub = sub;
    let n = sub.len() as u32;
    sub[1..5].copy_from_slice(&n.to_le_bytes());
    let mut f = vec![0u8; aim::SUBLAYER_OFFSET];
    f[..5].copy_from_slice(b"AIM\0\x03");
    f[5..9].copy_from_slice(&((aim::SUBLAYER_OFFSET + sub.len() - 4) as u32).to_le_bytes());
    f[9..13].copy_from_slice(b"lala");
    f.extend_from_slice(&sub);
    f
}

/// Bloque `[u32 n][datos]`: comprimido o copia literal (cabecera negativa).
fn block(out: &mut Vec<u8>, data: &[u8], stored: bool) {
    let b = if stored {
        let mut b = (-(data.len() as i32)).to_le_bytes().to_vec();
        b.extend_from_slice(data);
        b
    } else {
        deflate(data)
    };
    out.extend_from_slice(&(b.len() as u32).to_le_bytes());
    out.extend_from_slice(&b);
}

fn main() {
    let dir = std::env::args().nth(1).expect("directorio");
    std::fs::create_dir_all(&dir).unwrap();
    let mut rng = Rng(0x5eed_a1a1);
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    // 0x21: banderas (bit 0 formato 3/4, bit 2 continuo) x profundidades.
    let depths: [[u8; 4]; 6] = [
        [8, 8, 8, 8],
        [5, 6, 5, 0],
        [5, 5, 5, 1],
        [4, 4, 4, 4],
        [8, 8, 8, 0],
        [3, 7, 2, 6],
    ];
    for flags in [0u8, 1, 2, 4, 5, 6, 7] {
        for (di, d) in depths.iter().enumerate() {
            let (w, h) = (3 + rng.next() % 90, 1 + rng.next() % 40);
            let pitch = w * 4;
            let px = rng.bytes((pitch * h) as usize, di % 2 == 0);
            let mut sub = vec![0u8; 0x16];
            sub[0] = 0x21;
            sub[5] = flags;
            sub[6..10].copy_from_slice(d);
            sub[10..14].copy_from_slice(&w.to_le_bytes());
            sub[14..18].copy_from_slice(&h.to_le_bytes());
            sub[18..22].copy_from_slice(&pitch.to_le_bytes());
            let stored = di == 3;
            if flags & 4 == 0 {
                for plane in 0..4 {
                    let p: Vec<u8> = px.chunks(4).map(|c| c[plane]).collect();
                    for c in p.chunks(0x10000) {
                        block(&mut sub, c, stored);
                    }
                }
            } else {
                for c in px.chunks(0x10000) {
                    block(&mut sub, c, stored);
                }
            }
            files.push((format!("x21_f{flags}_d{di}"), file(sub)));
        }
    }
    // Planos de más de 0x10000 bytes (varios bloques por plano).
    for flags in [0u8, 4] {
        let (w, h) = (300u32, 260u32);
        let px = rng.bytes((w * 4 * h) as usize, true);
        let mut sub = vec![0u8; 0x16];
        sub[0] = 0x21;
        sub[5] = flags;
        sub[6..10].copy_from_slice(&[8, 8, 8, 8]);
        sub[10..14].copy_from_slice(&w.to_le_bytes());
        sub[14..18].copy_from_slice(&h.to_le_bytes());
        sub[18..22].copy_from_slice(&(w * 4).to_le_bytes());
        if flags & 4 == 0 {
            for plane in 0..4 {
                let p: Vec<u8> = px.chunks(4).map(|c| c[plane]).collect();
                for c in p.chunks(0x10000) {
                    block(&mut sub, c, false);
                }
            }
        } else {
            for c in px.chunks(0x10000) {
                block(&mut sub, c, false);
            }
        }
        files.push((format!("x21_big_f{flags}"), file(sub)));
    }
    // 0x1a/0x1b/0x1c comprimidos y crudos.
    for kind in [0x1au8, 0x1b, 0x1c] {
        for raw in [false, true] {
            let (w, h) = (16u32 << (rng.next() % 3), 16u32 << (rng.next() % 3));
            let n = if kind == 0x1b { w * 4 * h / 8 } else { w * h } as usize;
            let px = rng.bytes(n, true);
            let mut sub = vec![0u8; 0xe];
            sub[0] = kind;
            sub[5] = u8::from(!raw);
            sub[6..10].copy_from_slice(&w.to_le_bytes());
            sub[10..14].copy_from_slice(&h.to_le_bytes());
            if raw {
                sub.extend_from_slice(&px);
            } else {
                sub.extend_from_slice(&(px.len() as u32).to_le_bytes());
                for c in px.chunks(0x4000) {
                    block(&mut sub, c, false);
                }
            }
            files.push((format!("x{kind:02x}_{}", if raw { "raw" } else { "z" }), file(sub)));
        }
    }
    // 0x16 con formato 5 y 6, y 0x22 con copia literal y paleta corta.
    for b5 in [1u8, 2] {
        let (w, h) = (7 + rng.next() % 50, 1 + rng.next() % 30);
        let pitch = (w * 3 + 3) & !3;
        let mut sub = vec![0u8; 0x12];
        sub[0] = 0x16;
        sub[5] = b5;
        sub[6..10].copy_from_slice(&w.to_le_bytes());
        sub[10..14].copy_from_slice(&h.to_le_bytes());
        sub[14..18].copy_from_slice(&pitch.to_le_bytes());
        sub.extend_from_slice(&rng.bytes((pitch * h) as usize, false));
        files.push((format!("x16_b{b5}"), file(sub)));
    }
    for (i, (stored, pal_len)) in [(true, 0x400usize), (false, 0x300), (true, 0x80)]
        .into_iter()
        .enumerate()
    {
        let (w, h) = (5 + rng.next() % 400, 1 + rng.next() % 300);
        let mut sub = vec![0u8; 0x15];
        sub[0] = 0x22;
        sub[5..9].copy_from_slice(&[8, 8, 8, 8]);
        sub[9..13].copy_from_slice(&w.to_le_bytes());
        sub[13..17].copy_from_slice(&h.to_le_bytes());
        sub[17..21].copy_from_slice(&w.to_le_bytes());
        block(&mut sub, &rng.bytes(pal_len, false), stored);
        for c in rng.bytes((w * h) as usize, true).chunks(0x10000) {
            block(&mut sub, c, stored);
        }
        files.push((format!("x22_{i}"), file(sub)));
    }
    let mut tsv = String::new();
    for (name, data) in &files {
        std::fs::write(format!("{dir}/{name}.aim"), data).unwrap();
        let img = aim::decode(data).unwrap_or_else(|e| panic!("{name}: {e}"));
        let n = if img.bytes_per_pixel == 0 {
            img.pitch as usize
        } else {
            (img.pitch * img.height) as usize
        };
        tsv.push_str(&format!(
            "{name}\t{}\t{}\t{}\t{}\t{}\t{n}\t{}\t{}\n",
            img.width,
            img.height,
            img.bytes_per_pixel,
            img.format,
            img.pitch,
            hex(&img.pixels[..n]),
            img.palette.as_deref().map(hex).unwrap_or_default()
        ));
    }
    std::fs::write(format!("{dir}/expected.tsv"), tsv).unwrap();
    eprintln!("{} archivos sintéticos", files.len());
}
