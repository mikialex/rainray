use crate::material::Material;
use crate::ray::*;
use crate::sphere::Sphere;

pub struct Model<T: Intersecterable> {
    pub geometry: T,
    pub material: Material,
}
