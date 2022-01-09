pub struct Window {
    pub context: sdl2::Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

pub fn init(width: u32, height: u32) -> Result<Window, String> {
    let context = sdl2::init()?;
    let video_sub = context.video()?;

    let win = match video_sub
        .window("sietor", width, height)
        .position_centered()
        .build()
    {
        Ok(win) => win,
        Err(error) => return Err(error.to_string()),
    };
    let canvas = match win.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(error) => return Err(error.to_string()),
    };
    let window = Window { context, canvas };
    Ok(window)
}
