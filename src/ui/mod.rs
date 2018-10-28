extern crate immi;

// Object that will allow you to draw the UI.
struct _MyDrawer;
impl immi::Draw for _MyDrawer {
    type ImageResource = str;
    type TextStyle = str;

    fn draw_triangle(&mut self, _: &str, _: &immi::Matrix, _: [[f32; 2]; 3]) {

    }
    fn get_image_width_per_height(&mut self, _: &str) -> f32 { 1.0 }
    fn draw_glyph(&mut self, _: &str, _: char, _: &immi::Matrix) { }
    fn line_height(&self, _: &str) -> f32 { 1.2 }
    fn kerning(&self, _: &str, _: char, _: char) -> f32 { 0.0 }
    fn glyph_infos(&self, _: &str, _: char) -> immi::GlyphInfos {
        immi::GlyphInfos { width: 1.0, height: 1.0, x_offset: 0.0,
                           y_offset: 1.0, x_advance: 1.0 }
    }
}

struct _MyUiState {
    immi_state: immi::UiState,
    widget1_text: String,
    checkbox: bool,
}

fn _draw_ui(_ctxt: &immi::DrawContext<_MyDrawer>, _ui_state: &mut _MyUiState) {
    // ...
}
