use crate::image::GenericImageView;
use crate::renderer::data;
use nalgebra as na;
use crate::{
    renderer::{buffer, GlProgram},
    resources::Resources,
};
use log::error;
use nalgebra::Matrix4;
use std::{ffi::CStr, f32::consts::PI};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::Vec3,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = 2]
    tex: data::Vec2,
}

pub struct Triangle {
    projection_mat: nalgebra::Matrix4<f32>,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    ebo: buffer::ElementArrayBuffer,
    program: GlProgram,
    texture: gl::types::GLuint,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Self, failure::Error> {
        let program = GlProgram::from_res(gl, res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (100.0, 100.0, 0.0).into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
                tex: (1.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (100.0, 50.0, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
                tex: (1.0, 0.0).into(),
            }, // bottom left
            Vertex {
                pos: (50.0, 50.0, 0.0).into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
                tex: (0.0, 0.0).into(),
            }, // top
            Vertex {
                pos: (50.0, 100.0, 0.0).into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
                tex: (0.0, 1.0).into(),
            }, // top
        ];
        let indicies: Vec<u32> = vec![0, 1, 3, 1, 2, 3];
        let vbo = buffer::ArrayBuffer::new(&gl);
        let ebo = buffer::ElementArrayBuffer::new(&gl);
        let vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        vbo.static_draw_data(&vertices);

        ebo.bind();
        ebo.static_draw_data(&indicies);

        Vertex::vertex_attrib_pointers(&gl);

        vbo.unbind();
        vao.unbind();

        let mut texture = 0;
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        let img = image::open("./assets/imgs/container.jpg")?;
        let data = img.to_bytes();
        unsafe {
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const std::ffi::c_void,
            );
            gl.Uniform1i(
                gl.GetUniformLocation(program.id(), c_str!("tex").as_ptr()),
                0,
            );
            //gl.DebugMessageCallback(Triangle::cb_fn); huh
        }

        //let projection_mat: Matrix4<f32> = *nalgebra::Perspective3::new(16.0/9.0, 3.14 / 4.0, 0.1, 100.0).as_matrix();
        let projection_mat :Matrix4<f32>= *na::Orthographic3::new(0.0, 1920.0, 0.0, 1080.0, 0.1, 100.0).as_matrix();
//        let projection_mat = nalgebra_glm::ortho(0., 1920., 0., 1080., 0., 100.);


        Ok(Triangle {
            projection_mat ,
            vao,
            vbo,
            ebo,
            program,
            texture,
        })
    }
    pub fn draw(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            let err = gl.GetError();
            if err != gl::NO_ERROR {
                error!("OpenGL error: {}", err);
            }

            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.texture);
            gl.ClearColor(25.0 / 255.0, 27.0 / 255.0, 28.0 / 255.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const core::ffi::c_void,
            );
        }
        //       self.window_context.swap_buffers().unwrap();
        self.vao.unbind();
    }
}
