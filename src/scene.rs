use crate::light::*;
use crate::model::*;

pub struct Scene {
    pub models: Vec<Model>,
    pub lights: Vec<PointLight>,
}
