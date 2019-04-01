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

impl BVHNode{
    pub fn build_from_range_primitives(
        primitive_list: &Vec<Primitive>,
         start: u64, count: u64
        ) -> BVHNode{
            let bbox = get_range_primitives_bounding(primitive_list, start, count);
            return BVHNode{
                bounding_box: bbox,
                left: None,
                right: None,
                split_axis: get_longest_axis_of_bounding(&bbox),
                primitive_start: start,
                primitive_count: count,
            }
    }

    pub fn split(&mut self, primtive_list: &mut [Primitive], spliter: &Fn(&BVHNode) -> () ){
        // TODO opti
        match self.split_axis {
            Axis::x => primtive_list.sort_unstable_by(|a, b| a.cmp_center_x(b)),
            Axis::y => primtive_list.sort_unstable_by(|a, b| a.cmp_center_y(b)),
            Axis::z => primtive_list.sort_unstable_by(|a, b| a.cmp_center_z(b)),
        }

    }
}

fn build_equal_counts(node:&mut BVHNode){
    
}


pub struct Primitive {
    bounding_box: Box3,
    center_point: Vec3,
    index: u64,
}

impl Primitive {
    pub fn cmp_center_x(&self, other: &Primitive) -> std::cmp::Ordering {
        if self.center_point.x < other.center_point.x { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
    }

    pub fn cmp_center_y(&self, other: &Primitive) -> std::cmp::Ordering {
        if self.center_point.y < other.center_point.y { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
    }

    pub fn cmp_center_z(&self, other: &Primitive) -> std::cmp::Ordering {
        if self.center_point.z < other.center_point.z { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
    }
}


pub struct BVHAccel {
    root: BVHNode,
    primitives: Vec<Primitive>
}

impl BVHAccel{
    pub fn build(
        primitives: Vec<Primitive>
    ) -> BVHAccel {
        let mut bvh = BVHAccel {
            root: BVHNode::build_from_range_primitives(&primitives, 0, primitives.len() as u64),
            primitives
        };
        // bvh.root.split(&primitives, &build_equal_counts);
        bvh
    }


}

fn get_range_primitives_bounding(primitive_list: &Vec<Primitive>, start: u64, count: u64) -> Box3 {
    let mut bbox = primitive_list[start as usize].bounding_box.clone();
    for pid in start..(start + count) {
        bbox.extend_by_box(&primitive_list[pid as usize].bounding_box);
    }
    bbox
}

fn get_longest_axis_of_bounding(bbox: &Box3) -> Axis {
    let x_length = bbox.max.x - bbox.min.x;
    let y_length = bbox.max.y - bbox.min.y;
    let z_length = bbox.max.z - bbox.min.z;
    if x_length > y_length {
        if x_length > z_length {
            Axis::x
        } else {
            Axis::z
        }
    } else {
        if y_length > z_length {
            Axis::y
        } else {
            Axis::z
        }
    }
}
