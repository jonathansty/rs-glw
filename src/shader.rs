use super::*;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub enum ShaderType
{
    Vertex   ,
    Fragment,
    Compute ,
    Geometry 
}
impl ShaderType{
    fn value(&self) -> GLenum{
        match *self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER
        }
    }
}
/// A wrapper around opengl shader objects. 
pub struct Shader {
    id: u32,
    _shader_type: ShaderType
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping shader {}", self.id);

            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn get_id(&self) -> GLuint {
        self.id
    }
    // Creates a new empty shader object
    pub fn new(shader_type: ShaderType) -> Shader {
        unsafe {
            Shader {
                id: gl::CreateShader(shader_type.value()),
                _shader_type: shader_type
            }
        }
    }

    /// Loads and compiles a shader from memory.
    pub fn load_from_memory(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        unsafe {
            let shader_id = self.id;

            let c_string = CString::new(data)?;
            gl::ShaderSource(shader_id, 1, &c_string.as_ptr(), ::std::ptr::null());
            gl::CompileShader(shader_id);

            let mut success: GLint = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

            match success {
                0 => {
                    let mut log_size: GLint = 0;
                    gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_size);
                    let mut msg: Vec<u8> = Vec::new();
                    msg.resize(log_size as usize, 0);

                    let mut new_length = 0;
                    gl::GetShaderInfoLog(
                        shader_id,
                        log_size,
                        &mut new_length,
                        msg.as_ptr() as *mut GLchar,
                    );

                    let msg = format!(
                        "Failed to compile shader : {}",
                        String::from_utf8(msg).unwrap()
                    );

                    Err(From::from(msg))
                }
                _ => Ok(()), // Return empty OK
            }
        }
    }

    /// Loads and compiles a shader from a file on disk.
    pub fn load_from_file(&mut self, path: &str) -> Result<(), Box<dyn Error> > {
        let mut file = File::open(&path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)
            .expect("Failed to read from file");

        let c_content: CString = CString::new(content.as_bytes()).unwrap();

        match c_content.to_str() {
            Ok(v) => { 
                self.load_from_memory(v)
            },
            Err(err) => {
                Err(From::from(format!("Failed to load shader for path \"{}\": \n{}", path, err))) 
            }
        }
    }
}


/// Enum that carries data for specific Uniforms in GLSL. 
/// It describes what data to bind for the uniform retrieved with glGetUniformLocation
#[allow(dead_code)]
pub enum Uniform {
    Float(f32),
    Int(i32),
    Vec2(f32,f32),
    Sampler2D(GLuint),
}
