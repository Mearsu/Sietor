use nalgebra as na;
use gl;

pub struct ColorBuffer{
		pub color: na::Vector4<f32>,
}

impl ColorBuffer{
		pub fn from_color(color: na::Vector3<f32>) -> Self{
				ColorBuffer{
						color: color.fixed_resize(1.0),
				}
		}
	#[allow(dead_code)]
		pub fn update_color(&mut self, color: na::Vector3<f32>){
				self.color = color.fixed_resize(1.0);
		}

		pub fn set_used(&self, gl: &gl::Gl){
				unsafe{
						gl.ClearColor(self.color.x, self.color.y, self.color.z, self.color.w);
				}
		}
		pub fn clear(&self, gl: &gl::Gl){
				unsafe{
						gl.Clear(gl::COLOR_BUFFER_BIT);
				}
		}

}
