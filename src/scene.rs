use crate::light::*;
use crate::model::*;
use crate::environment::*;

pub struct Scene {
    pub models: Vec<Model>,
    pub point_lights: Vec<PointLight>,
    pub env: Box<Environment>
}
