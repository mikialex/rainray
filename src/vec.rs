#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(vec_a: &Vec3, vec_b: &Vec3) -> f64 {
        vec_a.x * vec_b.x + vec_a.y * vec_b.y + vec_a.z * vec_b.z
    }

    pub fn cross(vec_a: &Vec3, vec_b: &Vec3) -> Vec3 {
        Vec3 {
            x: vec_a.y * vec_b.z - vec_a.z * vec_b.y,
            y: vec_a.x * vec_b.z - vec_a.z * vec_b.x,
            z: vec_a.x * vec_b.y - vec_a.y * vec_b.x,
        }
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&mut self) -> &Self {
        let inv_len = self.length().recip();
        self.x = self.x * inv_len;
        self.y = self.y * inv_len;
        self.z = self.z * inv_len;
        return self
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

impl std::ops::Mul<f64> for Vec3 {
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    type Output = Vec3;
}

impl std::ops::Div<f64> for Vec3 {
    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

    type Output = Vec3;
}