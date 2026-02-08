use core::fmt::Display;
use core::ops::{Div, Mul};

#[derive(Copy, Clone)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32,
}
impl Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output
    {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}
impl Mul<f32> for &Vector2
{
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output
    {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<f32> for Vector2
{
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output
    {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Vector2
{
    pub fn new(x: f32, y: f32) -> Self
    {
        Self { x, y }
    }

    pub fn length(&self) -> f32
    {
        libm::sqrtf(self.x * self.x + self.y * self.y)
    }

    pub fn normalized(&self) -> Self
    {
        self / self.length()
    }
}