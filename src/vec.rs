use std::io::{Write};
use crate::interval::Interval;
use crate::utility::{random_double, random_double_in_range};

#[derive(Clone,Copy,Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    // Instantiation and custom access
    pub fn with_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }
    
    // Vector operations
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Vec3 {
        Vec3 {e: [random_double(), random_double(), random_double()] }
    }

    pub fn random_with_range(min: f64, max: f64) -> Vec3 {
        Vec3 {e: [
            random_double_in_range(min, max),
            random_double_in_range(min, max),
            random_double_in_range(min, max)]}
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p: Vec3 = Vec3::random_with_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn close_to_zero(&self) -> bool {
        let tolerance: f64 = 1e-8;
        return (self.e[0] < tolerance) && (self.e[1] < tolerance) && (self.e[2] < tolerance);
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

// Basic Operations
impl std::ops::Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl std::ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl std::ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

// Generic implementation of division by a scalar
impl<T> std::ops::Div<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn div(self, scalar: T) -> Vec3 {
        let scalar_f64: f64 = scalar.into();

        Vec3 {
            e: [
                self.e[0] / scalar_f64,
                self.e[1] / scalar_f64,
                self.e[2] / scalar_f64,
            ],
        }
    }
}

// Generic implementation of multiplication by a scalar
impl<T> std::ops::Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn mul(self, scalar: T) -> Vec3 {
        let scalar_f64: f64 = scalar.into();

        Vec3 {
            e: [
                self.e[0] * scalar_f64,
                self.e[1] * scalar_f64,
                self.e[2] * scalar_f64,
            ],
        }
    }
}

// Assignment operators
impl<T> std::ops::MulAssign<T> for Vec3
where
    T: Into<f64> + Copy,
{
    fn mul_assign(&mut self, scalar: T) {
        let scalar_f64: f64 = scalar.into();
        self.e[0] *= scalar_f64;
        self.e[1] *= scalar_f64;
        self.e[2] *= scalar_f64;
    }
}

impl<T> std::ops::AddAssign<T> for Vec3
where
    T: Into<f64> + Copy,
{
    fn add_assign(&mut self, scalar: T) {
        let scalar_f64: f64 = scalar.into();
        self.e[0] += scalar_f64;
        self.e[1] += scalar_f64;
        self.e[2] += scalar_f64;
    }
}

impl<T> std::ops::SubAssign<T> for Vec3
where
    T: Into<f64> + Copy,
{
    fn sub_assign(&mut self, scalar: T) {
        let scalar_f64: f64 = scalar.into();
        self.e[0] -= scalar_f64;
        self.e[1] -= scalar_f64;
        self.e[2] -= scalar_f64;
    }
}

impl<T> std::ops::DivAssign<T> for Vec3
where
    T: Into<f64> + Copy,
{
    fn div_assign(&mut self, scalar: T) {
        let scalar_f64: f64 = scalar.into();
        self.e[0] /= scalar_f64;
        self.e[1] /= scalar_f64;
        self.e[2] /= scalar_f64;
    }
}

// Default access operators
impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

// Operations between vectors
fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3{
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0]
        ],
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length() 
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2] 
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = Vec3::random_unit_vector();
    if dot(&on_unit_sphere, &normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return on_unit_sphere * -1;
    }
}

// Reflect an incoming ray across the surface, using its normal
// The normal is assumed to be of unit length.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - *n * 2.0 * dot(&v, &n);
}

// Writing color to an image from a Vec3 struct
pub fn write_color(out: &mut dyn Write, pixel_color: Vec3, samples_per_pixel: i16) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide each color channel by the number of samples
    let scale: f64 = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let intensity = Interval::new(0.0, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as i16;
    let ig = (256.0 * intensity.clamp(g)) as i16;
    let ib = (256.0 * intensity.clamp(b)) as i16;
    writeln!(out, "{} {} {}", ir, ig, ib).expect("Failed to write color!");
}
