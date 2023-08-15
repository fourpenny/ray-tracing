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
    fn double_dot(&self, other: Vec3) -> Vec3 {
    }

    fn cross(&self, other: Vec3) -> Vec3 {
    }

    fn unit_vector(&self) -> Vec3 {
    }
}

// Basic Operations
impl std::ops::Add for Vec3 {
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

// Default access operators

