pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3{
    pub fn dot(vecA: &Vec3, vecB: &Vec3) -> f32 {
        vecA.x * vecB.x +  vecA.y * vecB.y + vecA.z * vecB.z
    }

    pub fn cross(vecA: &Vec3, vecB: &Vec3) -> Vec3{
        Vec3{
            x: vecA.y * vecB.z - vecA.z * vecB.y,
            y: vecA.x * vecB.z - vecA.z * vecB.x,
            z: vecA.x * vecB.y - vecA.y * vecB.x,
        }
    }
}

impl std::ops::Add for Vec3 {
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    type Output = Vec3;
}

impl std::ops::Mul<f32> for Vec3 {
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    type Output = Vec3;
}
