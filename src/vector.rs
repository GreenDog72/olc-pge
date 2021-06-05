use std::ops::{
    Add, Sub, Mul, Div, Neg,
    AddAssign, SubAssign, MulAssign, DivAssign
};

#[derive(Copy, Clone, Debug)]
pub struct V2d<T> {
    pub x: T,
    pub y: T
}

pub type Vi2d = V2d<i32>;
pub type Vf2d = V2d<f32>;
pub type Vd2d = V2d<f64>;

impl<T> V2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl V2d<f32> {
    pub fn mag(self) -> f32 {
        self.mag2().sqrt()
    }

    pub fn norm(self) -> Self {
        let r = 1.0 / self.mag();
        Self {
            x: self.x * r,
            y: self.y * r
        }
    }
}

impl V2d<f64> {
    pub fn mag(self) -> f64 {
        self.mag2().sqrt()
    }

    pub fn norm(self) -> Self {
        let r = 1.0 / self.mag();
        Self {
            x: self.x * r,
            y: self.y * r
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> V2d<T> {
    pub fn mag2(self) -> T {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn dot(self, rhs: &Self) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> V2d<T> {
    pub fn cross(self, rhs: &Self) -> T {
        (self.x * rhs.x) - (self.y * rhs.y)
    }
}

impl<T: Neg<Output = T>> V2d<T> {
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x
        }
    }
}

impl<T: Default> Default for V2d<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default()
        }
    }
}

impl<T: Neg<Output = T>> Neg for V2d<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<T: Add<Output = T>> Add for V2d<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for V2d<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for V2d<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for V2d<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for V2d<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for V2d<T> {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for V2d<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other
        }
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for V2d<T> {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other
        }
    }
}