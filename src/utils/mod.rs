use std;

pub fn clamp<T : std::cmp::PartialOrd>(x : T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub type Unit = f32;

pub fn deg_to_rad(theta: Unit) -> Unit {
    const DEG_TO_RAD_C : f32 = 3.14159265359 / 180.0;
    theta * DEG_TO_RAD_C
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: Unit,
    pub y: Unit,
}

impl Vec2 {
    pub fn new(x: Unit, y: Unit) -> Self {
        Vec2{x,y}
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: Unit,
    pub y: Unit,
    pub z: Unit,
}


#[derive(Clone, Copy)]
pub struct Mat33 {
    pub data: [Unit; 9],
}

impl Mat33 {
    pub fn new(data: [Unit;9]) -> Self {
        Mat33{data}
    }

    pub fn new_rotation(theta: Unit) -> Self {
        let theta_rad = deg_to_rad(theta);
        let ct = theta_rad.cos();
        let st = theta_rad.sin();
        Mat33{data: [
            ct, st, 0.0,
            -st, ct, 0.0,
            0.0, 0.0, 1.0,
        ]}
    }
    pub fn new_translation(pos: Vec2) -> Self {
        Mat33{data: [
            1.0, 0.0, pos.x,
            0.0, 1.0, pos.y,
            0.0, 0.0, 1.0,
        ]}
    }

    pub fn identity() -> Self {
        Mat33{data: [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]}
    }
}

impl std::ops::Mul<Mat33> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Mat33) -> Vec2 {
        let x = self.x * rhs.data[0] + self.y * rhs.data[1] + rhs.data[2];
        let y = self.x * rhs.data[3] + self.y * rhs.data[4] + rhs.data[5];
        let z = self.x * rhs.data[6] + self.y * rhs.data[7] + rhs.data[8];
        Vec2{x: x/z, y: y/z}
    }
}

impl std::ops::Mul<Mat33> for Mat33 {
    type Output = Mat33;

    fn mul(self, rhs: Mat33) -> Mat33 {
        Mat33{data: [
            self.data[0] * rhs.data[0] + self.data[1] * rhs.data[3] + self.data[2] * rhs.data[6],
            self.data[0] * rhs.data[1] + self.data[1] * rhs.data[4] + self.data[2] * rhs.data[7],
            self.data[0] * rhs.data[2] + self.data[1] * rhs.data[5] + self.data[2] * rhs.data[8],
            self.data[3] * rhs.data[0] + self.data[4] * rhs.data[3] + self.data[5] * rhs.data[6],
            self.data[3] * rhs.data[1] + self.data[4] * rhs.data[4] + self.data[5] * rhs.data[7],
            self.data[3] * rhs.data[2] + self.data[4] * rhs.data[5] + self.data[5] * rhs.data[8],
            self.data[6] * rhs.data[0] + self.data[7] * rhs.data[3] + self.data[8] * rhs.data[6],
            self.data[6] * rhs.data[1] + self.data[7] * rhs.data[4] + self.data[8] * rhs.data[7],
            self.data[6] * rhs.data[2] + self.data[7] * rhs.data[5] + self.data[8] * rhs.data[8],
        ]}
    }
}

pub trait Newable {
    fn new() -> Self;
}

