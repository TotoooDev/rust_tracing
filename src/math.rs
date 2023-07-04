pub mod vec3;
pub mod ray;

pub const PI: f64 = 3.1415926535897932385;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}