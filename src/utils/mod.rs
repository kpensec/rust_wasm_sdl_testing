extern crate sdl2;
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

pub struct Vec2 {
    pub x: Unit,
    pub y: Unit,
}

// pub union Rect {
//     {
//         pub pos: Vec2,
//         pub size: Vec2,
//     }
//     pub struct {
//         x: Unit,
//         y: Unit
//     }
// }

