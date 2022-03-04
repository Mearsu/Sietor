use nalgebra as na;

use super::GlProgram;

pub struct Camera {
    view_mat: na::Matrix4<f32>,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

impl Camera {
    pub fn view_mat(&self) -> &na::Matrix4<f32> {
        &self.view_mat
    }
		pub fn use_mat(&self,name:&str, program: GlProgram){
				program.set_mat4(name, *self.view_mat())
		}
    pub fn update(&mut self) {
        self.view_mat =
            *na::Orthographic3::new(self.x as f32, self.width as f32, self.y as f32, self.height as f32, 0.0, 100.0)
                .as_matrix();
    }

		pub fn new() -> Self{
				Camera{
						view_mat: na::Matrix4::identity(),
						width: 800,
						height: 600,
						x: 0,
						y:0,
				}
		}

    pub fn pos(&mut self, x: i32, y: i32) -> &mut Camera{
        self.x = x;
        self.y = y;
        self.update();
				self
    }
		pub fn set_size(&mut self, width: u32, height: u32) -> &mut Camera{
				self.width = width;
				self.height = height;
        self.update();
				self
		}
}
