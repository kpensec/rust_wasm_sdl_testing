use utils::{Vec2, Mat33};

// TODO not like this store a mesh and ext the transform?
// switch to gl renderering vs a soft rasterizer
pub struct Sprite {
    position: Vec2,
    size: Vec2,
    local_transform: Mat33,
    transform: Mat33,
}

impl Sprite {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Sprite {
            position,
            size,
            local_transform: Mat33::identity(),
            transform: Mat33::identity()
        }
    }

    pub fn calculate_transform(&mut self, t: Mat33) -> () {
        self.transform = t * self.local_transform;
    }
}

use render::{Renderer, RenderContext, RenderTarget};

impl Renderer for Sprite {
    fn render<T: RenderTarget>(&self, ctx: &mut RenderContext<T>) {
        let p = self.position.clone();
        // let t = self.transform.clone();
        let s = self.size.clone();
        ctx.draw_rect( p, s);
    }
}
