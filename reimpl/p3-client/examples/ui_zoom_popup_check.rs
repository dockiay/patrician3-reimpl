//! Local Windows check: real presentation of a zoomed city behind the church.
//! P3_EXE/P3_CPR and P3_UI_CAPTURE_DIR are required; no save is written.
use p3_client::{
    original,
    view::Script,
    window::{GameWindow, WindowConfig},
};
use p3_mvp::ui_rt::{menus, PageMem, Runtime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::fs::read(std::env::var_os("P3_EXE").ok_or("P3_EXE missing")?)?;
    let cpr = std::fs::read(std::env::var_os("P3_CPR").ok_or("P3_CPR missing")?)?;
    let out = std::path::PathBuf::from(std::env::var_os("P3_UI_CAPTURE_DIR").ok_or("P3_UI_CAPTURE_DIR missing")?);
    let archive = p3_assets::cpr::Archive::parse(&cpr)?;
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, &exe, &archive, 800, 600)?;
    rt.call_limit = 100_000_000;
    assert!(menus::boot(&mut rt, &mut mem));
    let mut cfg = WindowConfig::default();
    cfg.fullscreen = false;
    cfg.confine_cursor = false;
    cfg.system_cursor = true;
    cfg.title = "Patrician III - comprobacion de iglesia y zoom".into();
    let window = GameWindow::open(cfg)?;
    original::attach_scripted(&mut rt, window, Script::parse("10:mover 685 560;11:pulsar;12:soltar")?)?;
    rt.menus.frame_script = Some(Box::new(|m, f| {
        if f == 3 {
            m.w32(menus::MAIN_MENU + 0x284, 2);
        }
        f < 35
    }));
    menus::run(&mut rt, &mut mem);
    assert_eq!(rt.menus.frames, 35);
    rt.aborted = false;
    rt.call_limit = 100_000_000;
    rt.menus.frame_script = None;
    rt.call(&mut mem, 0x004C_0FA0, menus::MAIN_WINDOW, &[0, 480, 300, 300]);
    assert_eq!(rt.zoom.percent(), 200);
    let church = mem.u32_at(0x006E_54F0);
    rt.call(&mut mem, 0x0044_1180, church, &[0]);
    for _ in 0..75 {
        rt.call(&mut mem, 0x004B_8A40, menus::SCREEN_LIST, &[]);
    }
    assert!(!rt.aborted);
    let before = rt.surface.rgba.clone();
    rt.surface.write_png(&out.join("native-church-before.png"))?;
    for _ in 0..75 {
        rt.call(&mut mem, 0x004B_8A40, menus::SCREEN_LIST, &[]);
    }
    assert!(!rt.aborted);
    rt.surface.write_png(&out.join("native-church-after.png"))?;
    let mut changed = 0;
    for y in 200..700 {
        for x in 20..140 {
            let p = (y * rt.surface.width + x) * 4;
            changed += usize::from(before[p..p + 4] != rt.surface.rgba[p..p + 4]);
        }
    }
    assert!(changed > 0, "visible city patch froze in native presentation");
    eprintln!(
        "native church at 200%: {changed} changed city pixels, {} frames, aborted={}",
        rt.menus.frames, rt.aborted
    );
    Ok(())
}
