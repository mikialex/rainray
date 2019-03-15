use crate::bvh::*;

struct IndexMesh {
    index: Vec<u64>,
    position: Vec<f64>,
    normal: Vec<f64>,
}

impl IndexMesh {
    pub fn new(
        index:  Vec<u64>,
        position:  Vec<f64>,
        normal: Vec<f64>
        ) -> IndexMesh {
            IndexMesh {
                index,
                position,
                normal
            }
    }
}

fn gen_primitive_list(mesh: &IndexMesh) -> Vec<Primitive> {
    unimplemented!()
}

fn build_bvh_from_index_mesh(mesh: &IndexMesh) -> BVHAccel{
    unimplemented!()
}