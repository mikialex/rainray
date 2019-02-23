use crate::light::*;
use crate::model::*;
use crate::environment::*;

pub struct Scene {
    pub models: Vec<Model>,
    pub lights: Vec<PointLight>,
    pub env: Box<Environment>
}
