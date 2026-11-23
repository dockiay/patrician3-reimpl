use super::mp3_requantize::{self, RequantizeError};
use std::sync::LazyLock;

#[derive(Default)]
struct Node {
    children: [Option<usize>; 2],
    leaf: Option<(u16, u16)>,
}
static TREES: LazyLock<Vec<Vec<Node>>> = LazyLock::new(|| {
    let mut trees: Vec<Vec<Node>> = (0..34).map(|_| vec![Node::default()]).collect();
    for line in include_str!("mp3_huffman_tables.tsv")
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
    {
        let mut fields = line.split('\t');
        let table: usize = fields.next().unwrap().parse().unwrap();
        let bits = fields.next().unwrap();
        let x = fields.next().unwrap().parse().unwrap();
        let y = fields.next().unwrap().parse().unwrap();
        let tree = &mut trees[table];
        let mut i = 0;
        for bit in bits.bytes() {
            let b = (bit - b'0') as usize;
            i = if let Some(next) = tree[i].children[b] {
                next
            } else {
                let next = tree.len();
                tree.push(Node::default());
                tree[i].children[b] = Some(next);
                next
            };
        }
        assert!(tree[i].leaf.replace((x, y)).is_none());
    }
    trees
});
const LINBITS: [usize; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 8, 10, 13, 4, 5, 6, 7, 8, 9, 11, 13,
];

#[derive(Debug, Clone, Copy)]
pub struct HuffmanParams {
    pub start_bit: usize,
    pub end_bit: usize,
    pub regions: [usize; 3],
    pub tables: [u8; 3],
    pub count1_table: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HuffmanError {
    InvalidBounds,
    InvalidRegions,
    InvalidTable,
    Truncated,
    InvalidCode,
}
#[derive(Debug, Clone)]
pub struct HuffmanResult {
    pub magnitude: [u16; 576],
    pub sign: [u8; 576],
    pub used: usize,
    /// El original fija el cursor al final declarado aunque un cuádruple lo sobrepase.
    pub final_bit: usize,
    pub consumed_bit: usize,
}
struct Bits<'a> {
    data: &'a [u8],
    position: usize,
}
impl Bits<'_> {
    fn read(&mut self, n: usize) -> Result<u16, HuffmanError> {
        if self.position.checked_add(n).is_none_or(|end| end > self.data.len() * 8) {
            return Err(HuffmanError::Truncated);
        }
        let mut value = 0;
        for _ in 0..n {
            value = (value << 1) | u16::from((self.data[self.position / 8] >> (7 - self.position % 8)) & 1);
            self.position += 1;
        }
        Ok(value)
    }
    fn leaf(&mut self, table: usize) -> Result<(u16, u16), HuffmanError> {
        let tree = &TREES[table];
        let mut i = 0;
        loop {
            if let Some(leaf) = tree[i].leaf {
                return Ok(leaf);
            }
            i = tree[i].children[usize::from(self.read(1)?)].ok_or(HuffmanError::InvalidCode)?;
        }
    }
}
/// Decodifica los valores de una región ya delimitada por el lector de escalas.
/// Conserva los signos antiguos de los ceros que el original no vuelve a escribir.
pub fn decode(input: &[u8], p: HuffmanParams, prior_sign: [u8; 576]) -> Result<HuffmanResult, HuffmanError> {
    if p.start_bit > p.end_bit || p.end_bit > input.len().saturating_mul(8) {
        return Err(HuffmanError::InvalidBounds);
    }
    if p.regions[0] > p.regions[1]
        || p.regions[1] > p.regions[2]
        || p.regions[2] > 576
        || p.regions.iter().any(|r| r % 2 != 0)
    {
        return Err(HuffmanError::InvalidRegions);
    }
    if p.tables.iter().any(|&t| t >= 32) || p.count1_table > 1 {
        return Err(HuffmanError::InvalidTable);
    }
    let mut bits = Bits {
        data: input,
        position: p.start_bit,
    };
    let mut magnitude = [0; 576];
    let mut sign = prior_sign;
    let mut used = 0;
    for region in 0..3 {
        let table = usize::from(p.tables[region]);
        while used < p.regions[region] {
            if table == 0 {
                used += 2;
                continue;
            }
            let (x, y) = bits.leaf(table)?;
            for mut q in [x, y] {
                if q == 15 && LINBITS[table] != 0 {
                    q += bits.read(LINBITS[table])?;
                }
                magnitude[used] = q;
                sign[used] = if q != 0 { bits.read(1)? as u8 } else { 0 };
                used += 1;
            }
        }
    }
    while bits.position < p.end_bit && used < 576 {
        let (quad, _) = bits.leaf(32 + usize::from(p.count1_table))?;
        for shift in (0..4).rev() {
            let q = (quad >> shift) & 1;
            let s = if q != 0 { bits.read(1)? as u8 } else { 0 };
            if used < 576 {
                magnitude[used] = q;
                sign[used] = s;
            }
            used += 1;
        }
    }
    Ok(HuffmanResult {
        magnitude,
        sign,
        used: used.min(576),
        final_bit: p.end_bit,
        consumed_bit: bits.position,
    })
}

/// Conecta Huffman y requantización usando factores ya preparados por una fuente externa.
/// Todavía no calcula por sí mismo los índices de escala del flujo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpectrumError {
    InvalidMapping,
    Requantize(RequantizeError),
}
pub fn apply_prepared_gains(coeff: &HuffmanResult, map: &[(usize, usize, u32)]) -> Result<[u32; 576], SpectrumError> {
    let mut output = [0; 576];
    for &(input, dest, gain) in map {
        if input >= 576 || dest >= 576 {
            return Err(SpectrumError::InvalidMapping);
        }
        output[dest] = mp3_requantize::apply_magnitude(coeff.magnitude[input], gain, coeff.sign[input] != 0)
            .map_err(SpectrumError::Requantize)?;
    }
    Ok(output)
}
