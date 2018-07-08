
struct Text {
    size: f32,
    texture_handle: i32,
}

impl Text {
    pub fn new() -> Self {
        Text{1.0}
    }
}

impl Render for Text {
    fn render(&self, ctx: &mut RenderContext) {
    }
}

