use crate::frame::*;
use crate::ray::*;

pub struct SolidEnvironment{
    pub color: Color
}

pub trait Environment{
    fn sample_color(&self, ray: &Ray) -> Color;
}

impl Environment for SolidEnvironment{
    fn sample_color(&self, _ray: &Ray) -> Color{
        self.color
    }
}