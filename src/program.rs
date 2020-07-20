use super::*;

use std::ffi::CString;
use std::rc::Rc;

#[derive(Eq, PartialEq)]
pub enum PipelineType
{
    Graphics,
    Compute
}

/// # Pipeline
/// Interface to access internal graphics pipeline state
pub trait Pipeline {
    fn get_type(self: &Self) -> PipelineType;

    // Returns the API specific handle
    fn get(self : &Self) -> *const std::ffi::c_void;
}

#[derive(Default)]
pub struct GraphicsPipeline {
    // Open GL program ID
    id: GLuint,
}

// Implement the generic interface for pipelines 
impl Pipeline for GraphicsPipeline 
{
    fn get_type(self: &Self) -> PipelineType { PipelineType::Graphics }

    fn get(self: &Self) -> *const std::ffi::c_void
    {
        self.id as *const _
    }
}


impl GraphicsPipeline {
    fn new() -> GraphicsPipeline {
        unsafe {
            GraphicsPipeline {
                id: gl::CreateProgram()
            }
        }
    }


    pub fn set_uniform(&self, uniform_name: &str, uni: Uniform) {
        unsafe {
            let c_string : CString = CString::new(uniform_name).unwrap();
            let loc = gl::GetUniformLocation(self.id, c_string.as_ptr() as *const GLchar);
            match loc {
                -1 => {}
                _ => match uni {
                    Uniform::Float(v) => gl::Uniform1f(loc, v),
                    Uniform::Int(v) => gl::Uniform1i(loc, v),
                    Uniform::Vec2(x,y) => {
                        gl::Uniform2f(loc, x, y);
                    },
                    Uniform::Sampler2D(v) => {
                        self.set_sampler(v);

                        gl::Uniform1i(loc, v as i32);
                    }
                }
            }
        }
    }

    pub fn bind_buffer(&self, buffer: &impl buffers::BufferResource, slot : u32){
        unsafe{
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, slot, buffer.get_resource() as GLuint);
        }
    }

    pub fn bind_texture(&mut self, rt: &RenderTarget) {
        unsafe{
            gl::BindImageTexture(0, rt.get_texture(), 0, false as u8, 0, gl::WRITE_ONLY, gl::RGBA8);
        }
    }


    fn attach(&mut self, shader : &Shader)
    {
        unsafe {
            gl::AttachShader(self.id, shader.get_id());
        }
    }

    fn link(&mut self){
        unsafe{
            gl::LinkProgram(self.id);
        }
    }

    fn set_sampler(&self, sampler : GLuint){
        unsafe {
            // Bind our input texture
            gl::ActiveTexture(gl::TEXTURE0 + sampler);
            gl::BindTexture(gl::TEXTURE_2D, sampler);
        }
    }

}

#[derive(Default)]
pub struct PipelineBuilder{
    vshader: Option<Rc<Shader>>,
    fshader: Option<Rc<Shader>>,
    cshader: Option<Rc<Shader>>,
}

impl PipelineBuilder {
    pub fn new() -> PipelineBuilder{
        PipelineBuilder::default()
    }

    pub fn with_vertex_shader(&mut self, shader: Shader) -> &mut Self {
        self.vshader = Some(Rc::new(shader));

        self
    }

    pub fn with_fragment_shader(&mut self, shader: Shader) -> &mut Self {
        self.fshader = Some(Rc::new(shader));

        self
    }

    pub fn with_compute_shader(&mut self, shader: Shader) -> &mut Self {
        self.cshader = Some(Rc::new(shader));

        self
    }

    pub fn build(&self) -> GraphicsPipeline
    {
        let mut result = GraphicsPipeline::new();

        assert!(self.cshader.is_some() || ( (self.fshader.is_some() || self.vshader.is_some()) && self.cshader.is_none() ), "Can not have a compute shader bound to a program that has a vertex shader or a fragment shader!");

        if let Some(ref shader) = self.vshader {
            result.attach(&shader);
        }

        if let Some(ref shader) = self.fshader {
            result.attach(&shader);
        }

        if let Some(ref shader) = self.cshader {
            result.attach(&shader);
        }
        

        result.link();

        result
    }

}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}