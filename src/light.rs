use crate::vec::*;
use crate::frame::*;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3
}
