// structs suffixed with _float are normalized
#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vec3 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vec2 {
    pub d0: f32,
    pub d1: f32,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: ::vec_2_10_10_10::Vector,
}


impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3),
        }
    }
}

impl u2_u10_u10_u10_rev_float{
	pub unsafe fn vertex_attrib_pointer(gl:&gl::Gl, stride:usize, location:usize, offset:usize){
		gl.EnableVertexAttribArray(location as gl::types::GLuint);
		gl.VertexAttribPointer(
			location as gl::types::GLuint,
			4,//num of components per attribute
			gl::UNSIGNED_INT_2_10_10_10_REV,
			gl::TRUE,
			stride as gl::types::GLint,
			offset as *const gl::types::GLvoid
		);
	}
}

impl Vec3 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> Vec3 {
        Vec3 { d0, d1, d2 }
    }
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(other: (f32, f32, f32)) -> Self {
        Vec3::new(other.0, other.1, other.2)
    }
}


impl Vec2 {
    pub fn new(d0: f32, d1: f32) -> Vec2 {
        Vec2 { d0, d1 }
    }
    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        stride: usize,
        location: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(other: (f32, f32)) -> Self {
        Vec2::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_ {
    pub d0: i8,
}

impl i8_ {
    pub fn new(d0: i8) -> i8_ {
        i8_ { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribIPointer(location as gl::types::GLuint,
                                1, // the number of components per generic vertex attribute
                                gl::BYTE, // data type
                                stride as gl::types::GLint,
                                offset as *const gl::types::GLvoid);
    }
}

impl From<i8> for i8_ {
    fn from(other: i8) -> Self {
        i8_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_float {
    pub d0: i8,
}

impl i8_float {
    pub fn new(d0: i8) -> i8_float {
        i8_float { d0 }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(location as gl::types::GLuint,
                               1, // the number of components per generic vertex attribute
                               gl::BYTE, // data type
                               gl::TRUE, // normalized (int-to-float conversion)
                               stride as gl::types::GLint,
                               offset as *const gl::types::GLvoid);
    }
}

impl From<i8> for i8_float {
    /// Create this data type from i8
    fn from(other: i8) -> Self {
        i8_float::new(other)
    }
}
