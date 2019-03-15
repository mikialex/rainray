use crate::math::*;

#[derive(Debug, Clone)]
pub enum SplitMethod {
    SAH,
    Middle,
    EqualCounts,
}

pub enum Axis {
    x,
    y,
    z,
}

pub struct BVHNode {
    pub bounding_box: Box3,
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub split_axis: Axis,
    pub primitive_start: u64,
    pub primitive_count: u64,
}

pub struct Primitive {
    bounding_box: Box3,
    center_point: Vec3,
    index: u64,
}

pub struct BVHAccel {
    rootNode: Option<BVHNode>,
    primitives: Vec<Primitive>
}

impl BVHAccel{
    pub fn build(
        primitives: Vec<Primitive>
    ) -> BVHAccel {
        let mut bvh = BVHAccel {
            rootNode: None,
            primitives
        };
        bvh.build_equal_counts();
        bvh
    }

    fn build_equal_counts(&mut self){

    }

    fn build_equal_counts_recursive(
        &mut self,
        primitive_start: u64, 
        primitive_count: u64,
        ){
            
        }
}