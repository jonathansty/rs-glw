/// GLW is a opengl rust wrapper and utility library. Not suited for any production environment. 
/// The goal of this library is to provide basic "safe" rust interfaces for a user to write graphics programs faster.
extern crate gl;
extern crate glfw;

pub mod shader;
pub mod program;
pub mod color;
pub mod math;
pub mod rendertarget;
pub mod mesh;
pub mod glcontext;
pub mod buffers;

pub use self::mesh::{Mesh,MeshBuilder};
pub use self::program::{GraphicsPipeline,PipelineBuilder};
pub use self::math::Vec2;
pub use self::color::Color;
pub use self::shader::{Shader, Uniform};
pub use self::rendertarget::{RenderTarget};
pub use self::glcontext::{GLContext, MemoryBarrier};

use gl::types::*;
use std::os::raw::c_void;
use std::ffi::CStr;
