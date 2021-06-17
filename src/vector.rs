use std::ops::{
    Add, Sub, Mul, Div, Neg,
    AddAssign, SubAssign, MulAssign, DivAssign
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl<T: Add<Output = T>> Add<(T, T)> for V2d<T> {
    type Output = Self;

    fn add(self, other: (T, T)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign<(T, T)> for V2d<T> {
    fn add_assign(&mut self, other: (T, T)) {
        *self = Self {
            x: self.x + other.0,
            y: self.y + other.1
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

impl<T: Sub<Output = T>> Sub<(T, T)> for V2d<T> {
    type Output = Self;

    fn sub(self, other: (T, T)) -> Self {
        Self {
            x: self.x - other.0,
            y: self.y - other.1
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign<(T, T)> for V2d<T> {
    fn sub_assign(&mut self, other: (T, T)) {
        *self = Self {
            x: self.x - other.0,
            y: self.y - other.1
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

impl From<Vf2d> for Vi2d { fn from(value: Vf2d) -> Vi2d { Vi2d { x: value.x as i32, y: value.y as i32 } } }
impl From<Vd2d> for Vi2d { fn from(value: Vd2d) -> Vi2d { Vi2d { x: value.x as i32, y: value.y as i32 } } }

impl From<Vi2d> for Vf2d { fn from(value: Vi2d) -> Vf2d { Vf2d { x: value.x as f32, y: value.y as f32 } } }
impl From<Vd2d> for Vf2d { fn from(value: Vd2d) -> Vf2d { Vf2d { x: value.x as f32, y: value.y as f32 } } }

impl From<Vi2d> for Vd2d { fn from(value: Vi2d) -> Vd2d { Vd2d { x: value.x as f64, y: value.y as f64 } } }
impl From<Vf2d> for Vd2d { fn from(value: Vf2d) -> Vd2d { Vd2d { x: value.x as f64, y: value.y as f64 } } }

impl<T> From<(T, T)> for V2d<T> {
    fn from(value: (T, T)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl<T> From<V2d<T>> for (T, T) {
    fn from(value: V2d<T>) -> Self {
        (value.x, value.y)
    }
}