use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        if len > 0.0 {
            let inv_length = 1.0 / len;
            self.x = self.x * inv_length;
            self.y = self.y * inv_length;
            self.z = self.z * inv_length;
        }
    }

    pub fn dot(&self, vec: &Vec3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Vec3) -> Self {
        let x = self.y * vec.z - self.z * vec.y;
        let y = self.z * vec.x - self.x * vec.z;
        let z = self.x * vec.y - self.y * vec.x;
        Self { x: x, y: y, z: z }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub x: [f64; 4],
    pub y: [f64; 4],
    pub z: [f64; 4],
    pub w: [f64; 4],
}

impl Matrix {
    pub fn mul(&self, other: &Vec3) -> Vec3 {
        let mut x = self.x[0] * other.x + self.x[1] * other.y + self.x[2] * other.z + self.x[3];
        let mut y = self.y[0] * other.x + self.y[1] * other.y + self.y[2] * other.z + self.y[3];
        let mut z = self.z[0] * other.x + self.z[1] * other.y + self.z[2] * other.z + self.z[3];
        let w = self.w[0] * other.x + self.w[1] * other.y + self.w[2] * other.z + self.w[3];

        if w != 1.0 && w != 0.0 {
            x = x / w;
            y = y / w;
            z = z / w;
        }

        Vec3 { x, y, z }
    }
}

pub fn edge(vec_a: &Vec3, vec_b: &Vec3, point: &Vec3) -> f64 {
    (point.x - vec_a.x) * (vec_b.y - vec_a.y) - (point.y - vec_a.y) * (vec_b.x - vec_a.x)
}
