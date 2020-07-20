use super::*;

use super::color::Color;
use super::math::*;

use gl::types::*;

/// Available pixel formats for textures and render targets
pub enum PixelFormat{
   RGBA32F, 
   RGBA8,
   R8,
   R16
}

impl PixelFormat{

    /// Internal function to get the GL format for TexImage2D
    fn get_gl_format(&self) -> GLuint{
        match self {
            PixelFormat::RGBA32F | PixelFormat::RGBA8 => gl::RGBA,
            PixelFormat::R8 | PixelFormat::R16 => gl::RED,
        }
    }

    /// Internal function to get the actual internal format 
    fn get_gl_internal(&self) -> GLuint {
        match self {
            PixelFormat::RGBA32F => gl::RGBA32F,
            PixelFormat::RGBA8 => gl::RGBA8,
            PixelFormat::R8 => gl::R8,
            PixelFormat::R16 => gl::R16
        }
    }

    /// Internal function to get the pixel component type
    fn get_gl_type(&self) -> GLuint {
        match self {
            PixelFormat::RGBA32F => gl::FLOAT,
            PixelFormat::RGBA8 | PixelFormat::R8 => gl::UNSIGNED_BYTE,
            PixelFormat::R16 => gl::UNSIGNED_SHORT
        }
    }
}


#[allow(dead_code)]
pub struct RenderTarget{
    width : u32,
    height : u32,

    fmt: PixelFormat,

    fb: GLuint,
    tex: GLuint,
}

impl Default for RenderTarget{
    fn default() -> RenderTarget
    {
        RenderTarget{
            width:0,
            height:0,
            fb: 0,
            tex: 0,
            fmt: PixelFormat::RGBA32F,
        }
    }
}

impl RenderTarget{

    /// Creates a new render target with a specified size
    pub fn new(size : Vec2<u32>) -> Result<RenderTarget, &'static str> {
        let mut tex = 0;
        let mut fb = 0;
        let fmt = PixelFormat::RGBA32F;

        unsafe {
            gl::GenFramebuffers(1,&mut fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER,fb);

            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);

            gl::FramebufferTexture(gl::FRAMEBUFFER,gl::COLOR_ATTACHMENT0, tex,0);

            gl::TexImage2D(gl::TEXTURE_2D,
            0, 
            fmt.get_gl_internal() as i32,
            size.x as i32, 
            size.y as i32, 
            0, 
            fmt.get_gl_format() ,
            fmt.get_gl_type(),
            std::ptr::null());

            let complete = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::BindTexture(gl::TEXTURE_2D,0);

            if complete != gl::FRAMEBUFFER_COMPLETE {
                return Err("Render target creation failed. The framebuffer was not complete.");
            }
        }

        Ok( RenderTarget{
            width: size.x,
            height: size.y,
            fb,
            tex,
            fmt
        } )

    }

    /// Copies data from the CPU to the GPU memory
    pub fn map_data(&mut self, data : &Vec<Color> ) {
        unsafe{
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);

            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA8 as i32, self.width() as i32, self.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const std::os::raw::c_void);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER,0);
        }
    }

    /// Returns the internal OpenGL handle to the framebuffer
	pub fn get_fb(&self) -> GLuint{
		self.fb
	}

    /// Returns the internal OpenGL handle to the texture
    pub fn get_texture(&self) -> GLuint
    {
        self.tex
    }

    pub fn width(&self) -> u32{
        self.width
    }
    pub fn height(&self) -> u32{
        self.height
    }

}
impl Drop for RenderTarget
{
    fn drop(&mut self){
        unsafe{
            gl::DeleteFramebuffers(1, &self.fb);
            gl::DeleteTextures(1, &self.tex);
        }
    }
}
