use p3_mvp::ui_rt::{menus, win32, PageMem, Runtime};

fn main() {
    let exe = std::fs::read(std::env::var_os("P3_EXE").unwrap()).unwrap();
    let cpr = std::fs::read(std::env::var_os("P3_CPR").unwrap()).unwrap();
    let archive = p3_assets::cpr::Archive::parse(&cpr).unwrap();
    let mut mem = PageMem::default();
    let mut rt = Runtime::new(&mut mem, &exe, &archive, 800, 600).unwrap();
    rt.call_limit = 100_000_000;
    rt.input.time_ms = Some(0);
    assert!(menus::boot(&mut rt, &mut mem));
    // Diagnostic reproduction of the formerly skipped CRT initializer.
    // This flag is local to this example, never part of the native client.
    if std::env::args().any(|a| a == "--legacy-wall-init") { mem.write(0x6e3fd8, &[0;10]); }
    println!("wall codes after boot: {:02x?}", mem.read(0x6e3fd8,10));
    let mut frame = 0;
    rt.on_present = Some(Box::new(move |_, queue, input| {
        frame += 1;
        input.time_ms = Some(frame.min(35) * 40);
        if frame == 10 {
            input.cursor = (685, 561);
            win32::post(queue, input, win32::WM_MOUSEMOVE, 0, win32::mouse_lparam(685, 561));
        }
        if frame == 11 || frame == 12 {
            let down = frame == 11;
            input.set_key(win32::VK_LBUTTON, down);
            win32::post(queue, input, if down {win32::WM_LBUTTONDOWN} else {win32::WM_LBUTTONUP}, input.mouse_keys(), win32::mouse_lparam(685, 561));
        }
        true
    }));
    rt.menus.frame_script = Some(Box::new(|m,f| {
        if f == 3 { m.w32(menus::MAIN_MENU + 0x284, 2); }
        if f == 30 {
            let scene=m.r32(0x6e51ac);
            let town=m.r32(0x6de508)+9*0x9f8;
            let records=m.r32(town+0x75c);
            for i in 0..u32::from(m.r16(town+0x776)) {
                let b=records+8*i;
                if m.r8(b+3)==0x36 && m.r8(b+7)==0x20 {
                    let x = u32::from(m.r8(b)) + m.r32(scene+0x528) - 5;
                    let y = u32::from(m.r8(b+1)) + m.r32(scene+0x52c) - 12;
                    m.w32(scene+0xc640, x);
                    m.w32(scene+0xc644, y);
                    break;
                }
            }
        }
        f < 40
    }));
    menus::run(&mut rt, &mut mem);
    let scene = mem.u32_at(0x6e51ac);
    let town = mem.u32_at(0x6de508) + 9*0x9f8;
    println!("scene {scene:x}, town {town:x}, frames {}, missing {:x?}", rt.menus.frames, rt.missing_from);
    println!("origin {:?} scroll {:?}", [mem.u32_at(scene+0x528),mem.u32_at(scene+0x52c)], [mem.u32_at(scene+0xc640),mem.u32_at(scene+0xc644)]);
    assert_ne!(scene, 0);
    let records = mem.u32_at(town+0x75c);
    let count = mem.u32_at(town+0x776) as u16;
    assert!(count > 0);
    for i in 0..u32::from(count) {
        let b=mem.read(records+8*i,8);
        if b[3]>=0x34 { println!("building {i}: {b:02x?}"); }
    }
    if let Some(dir)=std::env::var_os("P3_UI_CAPTURE_DIR") {rt.surface.write_png(&std::path::PathBuf::from(dir).join("wall-base.png")).unwrap();}
}
