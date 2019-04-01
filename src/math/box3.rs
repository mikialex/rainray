use crate::math::*;

#[derive(Debug, Clone, Copy)]
pub struct Box3{
    pub min: Vec3,
    pub max: Vec3,
}

impl Box3{
  pub fn extend_by_box(&mut self, the_other_box: &Box3){
    self.min.min(&the_other_box.min);
		self.max.max(&the_other_box.max);
  }
}
