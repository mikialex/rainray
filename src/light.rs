use crate::frame::*;
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
}
