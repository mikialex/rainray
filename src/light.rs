use crate::vec::*;
use crate::frame::*;


pub trait Light {
}

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Color
}

impl Light for PointLight {
}