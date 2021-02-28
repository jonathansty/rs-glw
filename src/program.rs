use super::*;

use std::ffi::CString;
use std::rc::Rc;

#[derive(Eq, PartialEq)]
pub enum PipelineType {
    Graphics,
    Compute,
}

/// # Pipeline
/// Interface to access internal graphics pipeline state
pub trait Pipeline {
    fn get_type(self: &Self) -> PipelineType;

    // Returns the API specific handle
    fn get(self: &Self) -> *const std::ffi::c_void;
}

#[derive(Default)]
pub struct GraphicsPipeline {
    // Open GL program ID
    id: GLuint,
}

// Implement the generic interface for pipelines
impl Pipeline for GraphicsPipeline {
    fn get_type(self: &Self) -> PipelineType {
        PipelineType::Graphics
    }

    fn get(self: &Self) -> *const std::ffi::c_void {
        self.id as *const _
    }
}

impl GraphicsPipeline {
    fn new() -> GraphicsPipeline {
        unsafe {
            GraphicsPipeline {
                id: gl::CreateProgram(),
            }
        }
    }

    fn attach(&mut self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.get_id());
        }
    }

    fn link(&mut self) {
        unsafe {
            gl::LinkProgram(self.id);
        }
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(Default)]
pub struct PipelineBuilder {
    vshader: Option<Rc<Shader>>,
    fshader: Option<Rc<Shader>>,
    cshader: Option<Rc<Shader>>,
}

impl PipelineBuilder {
    pub fn new() -> PipelineBuilder {
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

    pub fn build(&self) -> GraphicsPipeline {
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

pub trait CommandList {
    fn get(self: &Self) -> *const std::ffi::c_void;

    fn bind_buffer(&mut self, buffer: &impl buffers::BufferResource, slot: u32);

    fn bind_texture(&mut self, rt: &RenderTarget);

    fn set_uniform(&mut self, uniform_name: &str, uni: Uniform);

    // #TODO: Test
    fn set_sampler(&mut self, sampler: GLuint);

    /// Binds a pipeline to the context.
    fn bind_pipeline(&mut self, pipeline: &impl program::Pipeline);

    /// Set's the current active viewport
    fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32);

    /// Clears the current bound render target
    fn clear(&mut self, color: Option<Color>);

    /// Binds a render target for drawing
    fn bind_rt(&mut self, rt: &RenderTarget);

    fn dispatch(&mut self, groups_x: u32, groups_y: u32, groups_z: u32);

    fn memory_barrier(&mut self, barrier: MemoryBarrier);
}

#[derive(Default)]
pub struct GraphicsCommandList {
    current_pipeline: GLuint,
}

impl CommandList for GraphicsCommandList {
    fn get(self: &Self) -> *const std::ffi::c_void {
        std::ptr::null()
    }

    fn bind_buffer(&mut self, buffer: &impl buffers::BufferResource, slot: u32) {
        unsafe {
            gl::BindBufferBase(
                gl::SHADER_STORAGE_BUFFER,
                slot,
                buffer.get_resource() as GLuint,
            );
        }
    }

    fn bind_texture(&mut self, rt: &RenderTarget) {
        unsafe {
            gl::BindImageTexture(
                0,
                rt.get_texture(),
                0,
                false as u8,
                0,
                gl::WRITE_ONLY,
                gl::RGBA8,
            );
        }
    }

    fn set_uniform(&mut self, uniform_name: &str, uni: Uniform) {
        unsafe {
            let c_string: CString = CString::new(uniform_name).unwrap();
            let loc =
                gl::GetUniformLocation(self.current_pipeline, c_string.as_ptr() as *const GLchar);
            match loc {
                -1 => {}
                _ => match uni {
                    Uniform::Float(v) => gl::Uniform1f(loc, v),
                    Uniform::Int(v) => gl::Uniform1i(loc, v),
                    Uniform::Vec2(x, y) => {
                        gl::Uniform2f(loc, x, y);
                    }
                    Uniform::Sampler2D(v) => {
                        self.set_sampler(v);

                        gl::Uniform1i(loc, v as i32);
                    }
                },
            }
        }
    }

    // #TODO: Test
    fn set_sampler(&mut self, sampler: GLuint) {
        unsafe {
            // Bind our input texture
            gl::ActiveTexture(gl::TEXTURE0 + sampler);
            gl::BindTexture(gl::TEXTURE_2D, sampler);
        }
    }

    /// Binds a pipeline to the context.
    fn bind_pipeline(&mut self, pipeline: &impl program::Pipeline) {
        assert!(pipeline.get_type() == program::PipelineType::Graphics);
        unsafe {
            gl::UseProgram(pipeline.get() as GLuint);
        }

        // Keep track of the currently bound pipeline
        self.current_pipeline = pipeline.get() as GLuint;
    }

    /// Set's the current active viewport
    fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32){
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }

    /// Clears the current bound render target
    fn clear(&mut self, color: Option<Color>) {
        unsafe {
            match color {
                Some(c) => gl::ClearColor(
                    c.r as f32 / 255.0,
                    c.g as f32 / 255.0,
                    c.b as f32 / 255.0,
                    c.a as f32 / 255.0,
                ),
                None => {}
            }

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Binds a render target for drawing
    fn bind_rt(&mut self, rt: &RenderTarget) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, rt.get_fb());
        }
    }

    fn dispatch(&mut self, groups_x: u32, groups_y: u32, groups_z: u32) {
        unsafe {
            gl::DispatchCompute(groups_x as GLuint, groups_y as GLuint, groups_z as GLuint);
        }
    }

    fn memory_barrier(&mut self, barrier: MemoryBarrier) {
        unsafe { gl::MemoryBarrier(barrier.get()) }
    }
}
