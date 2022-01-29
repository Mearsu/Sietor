use crate::renderer::data;
use crate::{
    renderer::{buffer, GlProgram},
    resources::Resources,
};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::Vec3,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
    //clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    program: GlProgram,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Self, failure::Error> {
        let program = GlProgram::from_res(gl, res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
            }, // top
        ];
        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();
        Ok(Triangle { vao, vbo, program })
    }
    pub fn draw(&self, gl: &gl::Gl) {
        self.vao.bind();
				self.program.set_used();
        unsafe {
            gl.ClearColor(25.0 / 255.0, 27.0 / 255.0, 28.0 / 255.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
        //       self.window_context.swap_buffers().unwrap();
        self.vao.unbind();
    }
}
