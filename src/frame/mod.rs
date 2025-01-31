mod frame_decoration;
pub use frame_decoration::{
    nine_slice::FrameDecorationNineSlice, simple::FrameDecorationSimple, FrameDecoration,
};

use egui::{layers::ShapeIdx, Margin, Rect, Response, Sense, Shape, Ui, UiBuilder};

pub struct Frame<'a> {
    id_salt: &'a str,
    decoration: Option<FrameDecoration>,
    inner_margin: Margin,
    outer_margin: Margin,
}

impl<'a> Frame<'a> {
    pub fn new(
        id_salt: &'a str,
        decoration: FrameDecoration,
        inner_margin: Margin,
        outer_margin: Margin,
    ) -> Self {
        Frame {
            id_salt,
            decoration: Some(decoration),
            outer_margin,
            inner_margin,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, inner_content: impl FnOnce(&mut Ui)) -> Response {
        let background_placeholder = ui.painter().add(Shape::Noop);

        let inner_ui = self.setup_inner_ui(ui, inner_content);
        let area = inner_ui.min_rect() + self.inner_margin - self.outer_margin;

        let response = ui.allocate_rect(area, Sense::hover());

        self.paint(ui, background_placeholder, area);

        response
    }

    fn setup_inner_ui(&mut self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Ui {
        if let Some(FrameDecoration::NineSlice(nine_slice_decoration)) = &self.decoration {
            let border_size = nine_slice_decoration.texture.size / 4.0;

            self.inner_margin += border_size.x;
        }

        let inner_area = ui.available_rect_before_wrap() - self.inner_margin - self.outer_margin;
        let mut inner_ui = ui.new_child(UiBuilder::new().max_rect(inner_area));

        content(&mut inner_ui);

        inner_ui
    }

    fn paint(&mut self, ui: &mut Ui, where_to_paint: ShapeIdx, area: Rect) {
        let mut shapes: Vec<Shape> = vec![];
        let painter = ui.painter();

        if let Some(decoration) = self.decoration.take() {
            let shape = decoration.into_shape(ui, area, self.id_salt);

            shapes.push(shape);
        }

        for shape in shapes {
            painter.set(where_to_paint, shape);
        }
    }
}
