
use crate::model::*;
use crate::light::*;

pub struct Scene{
  pub models: Vec<Model>,
  pub lights: Vec<Box<Light>>
}