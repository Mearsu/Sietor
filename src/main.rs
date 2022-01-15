mod window;

//use std::{time::Duration, fmt::Result};

fn main() {
    match sietor() {
        Ok(it) => it,
        Err(err) => panic!("{}", err),
    };
    //    let mut event_pump = win.window.event_pump()?;
    //    let font_cont = font::init()?;
    //        let font = font::load_font(&font_cont, "./TerminusTTF.ttf", 14)?;
    //let font = font::load_font(&font_cont, "./SourceCodePro-Regular.otf", 12)?;

    //    let col = renderer::RGB {
    //        r: 24,
    //        g: 26,
    //        b: 27,
    //    };
    //    let mut buffer = String::new();
    //    'run: loop {
    //        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //    }
}

fn sietor() -> Result<(), String> {
    let win = window::WindowContext::new()?;

    win.run()?;
    //    loop {
    //        match win.draw() {
    //            Ok(it) => {
    //                if !it {
    //                    break;
    //                }
    //            }
    //            Err(err) => return Err(err),
    //        };
    //    }
    Ok(())
}
