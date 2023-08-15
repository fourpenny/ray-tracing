#[derive(Clone,Copy)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    // Instantiation and custom access
    fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    fn with_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    fn x(&self) -> f64 {
        self.e[0]
    }

    fn y(&self) -> f64 {
        self.e[1]
    }

    fn z(&self) -> f64 {
        self.e[2]
    }
    
    // Vector operations
    fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
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

fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length() 
}

fn double_dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2] 
}
