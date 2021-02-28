use super::*;

pub trait RenderBackend 
{

}

/// An openGL wrapper that is used to interface with openGL.
pub struct GLContext;


impl RenderBackend for GLContext
{

}

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
    pub fn set_debug(&mut self) {
        unsafe{
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(GLContext::gl_debug_message,std::ptr::null());
        }
    }

    pub fn create_command_list(&self) -> impl program::CommandList {
        program::GraphicsCommandList::default()
    }

    pub fn execute_command_list(&self, _ : &impl program::CommandList ) 
    {
        // Do nothing in opengl
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