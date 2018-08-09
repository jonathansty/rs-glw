use std::ops::{Add, Mul};
/// A basic struct to hold color RGBA_UINT color data
#[derive(Clone)]
pub struct Color
{
   pub r : u8,
   pub g : u8,
   pub b : u8,
   pub a : u8,
}

impl Default for Color{
	fn default() -> Color{
		Color{
			r: 0,g: 0,b: 0,a: 0
		}
	}
}

impl Color
{
    pub fn new(r:u8, g:u8, b:u8, a: u8) -> Color {
        Color{
            r, g, b, a
        }
    }
}

impl Add for Color
{
    type Output = Self;
    fn add(self, rhs : Color) -> Self{

       Color{
           r: self.r + rhs.r,
           g: self.g + rhs.g,
           b: self.b + rhs.b,
           a: self.a + rhs.a,
       } 
    }
}

impl Mul for Color{
    type Output = Self;
    fn mul(self, rhs : Self) -> Self
    {
        Color{
            r: self.r * rhs.r,
            b: self.b * rhs.g,
            g: self.g * rhs.b,
            a: self.a * rhs.a,
        }
    }
}
