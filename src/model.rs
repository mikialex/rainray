use crate::material::Material;
use crate::ray::*;

pub struct Model {
    pub geometry: Box<dyn Intersecterable>,
    pub material: Material,
}
