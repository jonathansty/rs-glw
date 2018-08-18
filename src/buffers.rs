use super::*;
use std::os::raw::c_void;

#[derive(Default)]
pub struct StructuredBuffer<T>{
	phantom: std::marker::PhantomData<T>,

	id : GLuint,
	buffer_size : usize
}

impl<T> StructuredBuffer<T>{
	pub fn get_id(&self) -> GLuint{
		self.id
	}

	pub fn get_size(&self) -> usize {
		self.buffer_size
		// std::mem::size_of::<T>() * self.data.len()
	}

	/// Creates and allocates a new SSBO for use with opengl
	pub fn new(data : Vec<T>) -> Self {
		// Creates the buffer
		let mut id = 0;
		let buffer_size = std::mem::size_of::<T>() * data.len();

		unsafe{
			gl::GenBuffers(1,&mut id);
			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, id);
			gl::BufferData(gl::SHADER_STORAGE_BUFFER, buffer_size as isize, data.as_ptr() as *const c_void, gl::DYNAMIC_COPY);
			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
		}

		StructuredBuffer{
			phantom: std::marker::PhantomData,
			id,
			buffer_size
		}
	}

	pub fn map_data(&mut self, data : &Vec<T>){
		unsafe{
			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);
			let d : *mut T = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_WRITE) as *mut T;

			std::ptr::copy(data.as_ptr() as *const T, d, self.buffer_size);

			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
		}

	}
}

impl<T> Drop for StructuredBuffer<T>{
	fn drop(&mut self){
		unsafe{
			gl::DeleteBuffers(1, &self.id);
		}
	}
}