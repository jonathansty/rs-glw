use super::*;

/// An openGL wrapper that is used to interface with openGL.
pub struct GLContext;

impl GLContext{

    /// Default debug message callback for opengl messages
    #[allow(unused_variables)]
    extern "system" fn gl_debug_message(source : GLenum, msg_type : GLenum, id : GLuint, severity : GLenum, length : GLsizei, message : *const GLchar, param : *mut c_void)
    {
        unsafe {
            let msg = CStr::from_ptr(message);
            println!("GL: {}", msg.to_str().unwrap());
        }
    }

    /// Creates anew OpenGL Context and links up the procedure address getter
    pub fn new(window : &mut glfw::Window) -> GLContext{
        gl::load_with(|s| window.get_proc_address(s) as *const _); 

        GLContext{}
    }

    /// Enables the OpenGL debug callbacks. This is only available in debug configurations
    pub fn set_debug(&mut self) -> &Self {
        unsafe{
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(GLContext::gl_debug_message,std::ptr::null());
        }

        self
    }

    /// Set's the current active viewport
    pub fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) -> &Self{
        unsafe{
            gl::Viewport(x,y,width,height);
        }

        self
    }

    /// Clears the current bound render target
    pub fn clear(&mut self, color : Option<Color>) -> &Self {
        unsafe {
            match color {
                Some(c) => gl::ClearColor(c.r as f32 / 255.0,c.g as f32 / 255.0,c.b as f32 / 255.0, c.a as f32 / 255.0),
                None => {}
            }

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self
    }

    pub fn bind_image(&mut self, rt: &RenderTarget)
    {
        unsafe{
            gl::BindImageTexture(0, rt.get_texture(), 0, false as u8, 0, gl::WRITE_ONLY, gl::RGBA8);
        }
    }

    /// Binds a shader program
    pub fn bind_pipeline(&mut self, program: &GraphicsPipeline){
        unsafe{
            gl::UseProgram(program.get_id());
        }

    }

    /// Binds a render target for drawing
    pub fn bind_rt(&mut self, rt: &RenderTarget){
       unsafe{
           gl::BindFramebuffer(gl::FRAMEBUFFER, rt.get_fb());
       }
       self.set_viewport(0,0,rt.width() as i32, rt.height() as i32);
    }

    pub fn dispatch_compute(&mut self, groups_x : u32, groups_y : u32, groups_z : u32){
        unsafe{
            gl::DispatchCompute(groups_x as GLuint, groups_y as GLuint, groups_z as GLuint);
        }
    }

    pub fn memory_barrier(&mut self, barrier : MemoryBarrier){
        unsafe{
            gl::MemoryBarrier(barrier.get())
        }
    }
}

pub enum MemoryBarrier {
    AtomicCounter,
    ShaderStorage,
    All,
}

impl MemoryBarrier{
    pub fn get(&self) -> GLuint{
        match self {
            MemoryBarrier::AtomicCounter => gl::ATOMIC_COUNTER_BARRIER_BIT,
            MemoryBarrier::ShaderStorage => gl::SHADER_STORAGE_BARRIER_BIT,
            _ => 0x0
        }
    }
}