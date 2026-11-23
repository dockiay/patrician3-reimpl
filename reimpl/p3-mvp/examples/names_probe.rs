fn main() {
    let a: Vec<String> = std::env::args().collect();
    let s = p3_mvp::Session::open(&std::fs::read(&a[1]).unwrap(), std::fs::read(&a[2]).unwrap()).unwrap();
    let st = s.state();
    println!(
        "{} {}",
        st["town_name"],
        st["destinations"]
            .as_array()
            .unwrap()
            .iter()
            .take(5)
            .map(|d| d["name"].to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
