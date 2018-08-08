extern crate gl;
extern crate glfw;

pub mod shader;
pub mod program;
pub mod color;
pub mod math;
pub mod rendertarget;
pub mod mesh;

pub use self::mesh::{Mesh,MeshBuilder};
pub use self::program::{GraphicsPipeline,PipelineBuilder};
pub use self::math::Vec2;
pub use self::color::Color;
pub use self::shader::{Shader, Uniform};
pub use self::rendertarget::{RenderTarget};

use gl::types::*;
use std::os::raw::c_void;
use std::ffi::CStr;

/// wrapper around raw opengl calls to interface with the graphics API
pub struct GLContext;

impl GLContext{

    #[allow(unused_variables)]
    extern "system" fn gl_debug_message(source : GLenum, msg_type : GLenum, id : GLuint, severity : GLenum, length : GLsizei, message : *const GLchar, param : *mut c_void)
    {
        unsafe {
            let msg = CStr::from_ptr(message);
            println!("GL: {}", msg.to_str().unwrap());
        }
    }

    pub fn new(window : &mut glfw::Window) -> GLContext{
        gl::load_with(|s| window.get_proc_address(s) as *const _); 

        GLContext{}
    }

    pub fn set_debug(&self) -> &Self {
        unsafe{
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(GLContext::gl_debug_message,std::ptr::null());
        }

        self
    }

    /// Set's the current active viewport
    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) -> &Self{
        unsafe{
            gl::Viewport(x,y,width,height);
        }

        self
    }

    /// Clears the current bound render target
    pub fn clear(&self, color : Option<Color>) -> &Self {
        unsafe {
            match color {
                Some(c) => gl::ClearColor(c.r as f32 / 255.0,c.g as f32 / 255.0,c.b as f32 / 255.0, c.a as f32 / 255.0),
                None => {}
            }

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self
    }

    /// Binds a shader program
    pub fn bind_pipeline(&self, program: &GraphicsPipeline){
        unsafe{
            gl::UseProgram(program.get_id());
        }

    }

    /// Binds a render target for drawing
    pub fn bind_rt(&self, rt: &RenderTarget){
       unsafe{
           gl::BindFramebuffer(gl::FRAMEBUFFER, rt.get_fb());
       }
    }
}