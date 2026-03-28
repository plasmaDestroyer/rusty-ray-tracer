use crate::rtweekend::random_f64;
use crate::rtweekend::random_f64_range;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Self {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            random_f64() * (max - min) + min,
            random_f64() * (max - min) + min,
            random_f64() * (max - min) + min,
        )
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

pub type Point3 = Vec3;

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3::new(self * vec.x, self * vec.y, self * vec.z)
    }
}

impl Mul<Vec3> for u32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        (self as f64) * vec
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self::Output {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Div<u32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: u32) -> Self::Output {
        self / scalar as f64
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_f64_range(-1.0, 1.0),
            random_f64_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(normal, &on_unit_sphere) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v, n) * 2.0
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}
