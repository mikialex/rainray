#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(vec_a: &Vec3, vec_b: &Vec3) -> f32 {
        vec_a.x * vec_b.x + vec_a.y * vec_b.y + vec_a.z * vec_b.z
    }

    pub fn cross(vec_a: &Vec3, vec_b: &Vec3) -> Vec3 {
        Vec3 {
            x: vec_a.y * vec_b.z - vec_a.z * vec_b.y,
            y: vec_a.x * vec_b.z - vec_a.z * vec_b.x,
            z: vec_a.x * vec_b.y - vec_a.y * vec_b.x,
        }
    }

    pub fn norm(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let inv_len = self.length().recip();
        Vec3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
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

impl std::ops::Sub for Vec3 {
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
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
