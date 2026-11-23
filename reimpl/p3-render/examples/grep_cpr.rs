use p3_assets::cpr::Archive;
fn main(){let a:Vec<String>=std::env::args().collect();let b=std::fs::read(&a[1]).unwrap();let ar=Archive::parse(&b).unwrap();let pat=a[2].as_bytes();
 for e in &ar.entries{if let Some(d)=ar.read(e){if d.len()<2_000_000 && d.windows(pat.len()).any(|w|w.eq_ignore_ascii_case(pat)){println!("{} {}",String::from_utf8_lossy(&e.name),d.len())}}}}
