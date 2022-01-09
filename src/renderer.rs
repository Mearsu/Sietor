use crate::window::Window;

pub trait Color {
    fn to_rgb(&self) -> RGB;
}
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color for RGB {
    fn to_rgb(&self) -> RGB {
        RGB {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
pub fn fill(win: &mut Window, col: &dyn Color) {
    let color = col.to_rgb();
    win.canvas
        .set_draw_color(sdl2::pixels::Color::RGB(color.r, color.g, color.b));
    win.canvas.clear();
}
