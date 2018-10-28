use utils::{Vec2};

pub struct Scene {
    size: Vec2
}

impl Scene {
    pub fn new(size: Vec2) -> Self {
        Scene{
            size
        }
    }
}


// impl Renderer for Scene {
// }
