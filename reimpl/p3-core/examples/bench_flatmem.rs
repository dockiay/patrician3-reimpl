//! Bancos de memoria paginada: allocate, blocks, hot y mixed; imprimir tiempo y checksum.
use p3_core::flatmem::{Mem,SparseMem};
use std::{hint::black_box,time::Instant};
#[inline(never)]
fn hot_loop(mem: &mut SparseMem, n: u32) -> u64 {
    let mut sum=0u64;
    for i in 0..n {
        let a=0x10000+(std::hint::black_box(i)&4095);
        let v=mem.r8(a); mem.w8(a,v.wrapping_add(1));
        sum=sum.wrapping_add(v as u64);
    }
    sum
}
fn main() {
    let mode=std::env::args().nth(1).unwrap_or_else(||"allocate".into());
    let mut sum=0u64;
    let start=Instant::now();
    match mode.as_str() {
        "allocate" => for _ in 0..16 {
            let mut mem=SparseMem::default();
            for p in 0..8192u32 { mem.seed(p*8192,black_box(&[17;64])); }
            sum=sum.wrapping_add(black_box(mem.r32(8191*8192)) as u64);
            black_box(&mem);
        },
        "blocks" => {
            let data:Vec<u8>=(0..16*1024*1024).map(|i|(i*37) as u8).collect();
            let mut out=vec![0u8;data.len()];
            for _ in 0..24 {
                let mut mem=SparseMem::default();
                mem.seed(0x10000,black_box(&data));
                mem.read_into(0x10000,black_box(&mut out));
                assert_eq!(out,data);
                sum=sum.wrapping_add(mem.r32(0x103ff) as u64);
                black_box(&mem);
            }
        },
        "hot" => {
            let mut mem=SparseMem::default(); mem.seed(0x10000,&[3;4096]);
            sum=hot_loop(&mut mem,100_000_000);
            black_box(mem);
        },
        "mixed" => { let mut mem=SparseMem::default(); for p in 0..2048u32 { mem.seed(p*8192,&[3;64]); } let mut rng=7u32; for _ in 0..1_000_000 { rng=rng.wrapping_mul(1664525).wrapping_add(1013904223); let a=((rng>>16)&2047)*8192+(rng&60); let v=mem.r32(black_box(a)); mem.w8(a,v.wrapping_add(1) as u8); sum=sum.wrapping_add(v as u64); } black_box(mem); },
        _=>panic!("unknown benchmark"),
    }
    println!("{{\"mode\":\"{mode}\",\"seconds\":{},\"checksum\":{sum}}}",start.elapsed().as_secs_f64());
}

