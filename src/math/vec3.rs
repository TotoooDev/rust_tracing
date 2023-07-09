use std::ops;
use rand::Rng;
use rand::random;

pub enum Axis {
    X, Y, Z
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    return Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x
    );
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * dot(v, n) * n;
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random::<f64>(), random::<f64>(), random::<f64>());
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        return Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
    }

    pub fn random_in_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_sphere = Vec3::random_in_sphere();
        if dot(in_sphere, normal) > 0.0 {
            return in_sphere
        }
        return -in_sphere;
    }

    pub fn random_unit() -> Vec3 {
        return Vec3::random_in_sphere().normalize();
    }

    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn normalize(self) -> Vec3 {
        return self / self.length();
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }
}

// Operator overloading
impl ops::Add::<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z);
    }
}
impl ops::AddAssign::<Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}
impl ops::Sub::<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z);
    }
}
impl ops::SubAssign::<Vec3> for Vec3 {
    fn sub_assign(&mut self, _rhs: Vec3) {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        self.z -= _rhs.z;
    }
}
impl ops::Mul::<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        return Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs);
    }
}
impl ops::Mul::<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        return Vec3::new(self * _rhs.x, self * _rhs.y, self * _rhs.z);
    }
}
impl ops::Mul::<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z);
    }
}
impl ops::MulAssign::<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}
impl ops::Div::<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        return Vec3::new(self.x / _rhs, self.y / _rhs, self.z / _rhs);
    }
}
impl ops::Div::<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        return Vec3::new(self / _rhs.x, self / _rhs.y, self / _rhs.z);
    }
}
impl ops::Div::<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x / _rhs.x, self.y / _rhs.y, self.z / _rhs.z);
    }
}
impl ops::DivAssign::<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3::new(-self.x, -self.y, -self.z);
    }
}