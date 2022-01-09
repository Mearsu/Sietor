pub struct Font<'ttf> {
    pub font: sdl2::ttf::Font<'ttf, 'ttf>,
    pub name: String,
    pub point_size: u16,
}

pub struct Context {
    context: sdl2::ttf::Sdl2TtfContext,
}

pub fn init() -> Result<Context, String> {
    match sdl2::ttf::init() {
        Ok(cont) => Ok(Context { context: cont }),
        Err(error) => Err(error.to_string()),
    }
}

pub fn load_font<'cont>(
    context: &'cont Context,
    path: &str,
    point_size: u16,
) -> Result<Font<'cont>, String> {
    let font = match context.context.load_font(path.clone(), point_size) {
        Ok(font) => font,
        Err(error) => return Err(error),
    };
    Ok(Font {
        font,
        name: path.to_string(),
        point_size,
    })
}
