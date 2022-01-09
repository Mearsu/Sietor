mod font;
mod renderer;
mod window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

fn main() -> Result<(), String> {
    let mut win = window::init(1920, 1080)?;
    let mut event_pump = win.context.event_pump()?;
    let font_cont = font::init()?;
    let font = font::load_font(&font_cont, "./TerminusTTF.ttf", 12)?;

    let col = renderer::RGB {
        r: 24,
        g: 26,
        b: 27,
    };
    let mut buffer = String::new();
    'run: loop {
        renderer::fill(&mut win, &col);
        //        win.canvas.from_surface(font.font.render("asdf")):
        if buffer.chars().count() > 0 {
            let surf = match font
                .font
                .render(buffer.as_str())
                .solid(sdl2::pixels::Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                }) {
                Ok(srf) => srf,
                Err(error) => panic!("{}", error),
            };
            let tc = win.canvas.texture_creator();
            let screen_position = Point::new((surf.width() / 2) as i32, (surf.height() / 2) as i32);
            let screen_rect = Rect::from_center(screen_position, surf.width(), surf.height());
            match win
                .canvas
                .copy(&surf.as_texture(&tc).unwrap(), None, screen_rect)
            {
                Ok(_) => {}
                Err(error) => {
                    panic!("{}", error);
                }
            };
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'run;
                }
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(kc) => {
                        if kc as i32 >= Keycode::A as i32 && kc as i32 <= Keycode::Z as i32 {
                            buffer.push_str(kc.to_string().as_str());
                        }
                        match kc {
                            Keycode::Backspace => {
                                buffer.pop();
                            }
                            Keycode::Space => {
                                buffer.push(' ');
                            }
                            _ => {}
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
        win.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
