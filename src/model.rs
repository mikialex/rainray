use crate::material::Material;
use crate::ray::*;

pub struct Model {
    pub geometry: Box<Intersecterable>,
    pub material: Material,
}
