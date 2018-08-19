use super::*;
use std::os::raw::c_void;

/// # Structured Buffers
/// Simple wrapper around SSBO objects, This is a generic type as this is a type that would
/// be used in conjunction with a simple data structure defined in code.
/// This structured will then be mirrored in the shader programs and accessed.
/// Once the data is mapped the buffer does not keep a copy of the CPU data. 
/// This is up to the user to track and resubmit whenever a change occurs.
#[derive(Default)]
pub struct StructuredBuffer<T>
		where T: Default + Clone {
	phantom: std::marker::PhantomData<T>,

	id : GLuint,
	buffer_size : usize,
	elements : usize,
}

impl<T: Default + Clone> StructuredBuffer<T> {

	/// Creates and allocates a new empty structured buffer for use on the GPU. 
	/// ```size``` is the amount of elements. To Map data to the buffer use ```map_data(...)```
	pub fn new(size : usize) -> Self 
	{
		// Creates the buffer
		let mut id = 0;
		let buffer_size = std::mem::size_of::<T>() * size;

		let mut empty_data = Vec::new();
		empty_data.resize(size, T::default());

		unsafe{
			gl::GenBuffers(1,&mut id);

			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, id);
			gl::BufferData(gl::SHADER_STORAGE_BUFFER, buffer_size as isize, empty_data.as_ptr() as *const c_void, gl::DYNAMIC_COPY);
			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
		}

		StructuredBuffer{
			phantom: std::marker::PhantomData,
			id,
			buffer_size,
			elements: size
		}
	}

	/// Creates and allocates a new structured buffer with struct of type T and the provided data
	pub fn from(data : Vec<T>) -> Self {
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
			buffer_size,
			elements: data.len()
		}
	}

	/// Simple way to copy data from a Vec to the GPU memory. 
	/// The input vector needs to be correctly sized or else this method will panic!
	pub fn map_data(&mut self, data : &Vec<T>){
		unsafe{
			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.id);

			let d : *mut T = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::WRITE_ONLY) as *mut T;
			std::ptr::copy(data.as_ptr() as *const T, d, self.elements);
			gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);

			gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
		}

	}

	// Returns the internal OpenGL buffer id
	pub fn get_id(&self) -> GLuint{
		self.id
	}

	pub fn get_size(&self) -> usize {
		self.buffer_size
	}
}

impl<T: Default + Clone> Drop for StructuredBuffer<T>{
	fn drop(&mut self){
		unsafe{
			gl::DeleteBuffers(1, &self.id);
		}
	}
}