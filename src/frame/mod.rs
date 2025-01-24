mod frame_decoration;
pub use frame_decoration::{
    nine_slice::FrameDecorationNineSlice, simple::FrameDecorationSimple, FrameDecoration,
};

use egui::{layers::ShapeIdx, Margin, Rect, Response, Sense, Shape, Ui, UiBuilder};

pub struct Frame<'a> {
    id_salt: Option<&'a str>,
    decoration: Option<FrameDecoration<'a>>,
    inner_margin: Margin,
    outer_margin: Margin,
}

impl<'a> Frame<'a> {
    pub fn new(
        decoration: FrameDecoration<'a>,
        inner_margin: Margin,
        outer_margin: Margin,
    ) -> Self {
        Frame {
            id_salt: None,
            decoration: Some(decoration),
            outer_margin,
            inner_margin,
        }
    }

    pub fn id_salt(mut self, id_salt: &'a str) -> Self {
        self.id_salt = Some(id_salt);

        self
    }

    pub fn show(&mut self, ui: &mut Ui, inner_content: impl FnOnce(&mut Ui)) -> Response {
        let background_placeholder = ui.painter().add(Shape::Noop);

        let inner_ui = self.setup_inner_ui(ui, inner_content);
        let area = inner_ui.min_rect() + self.inner_margin - self.outer_margin;

        let response = ui.allocate_rect(area, Sense::hover());

        self.paint(ui, background_placeholder, area);

        response
    }

    fn setup_inner_ui(&self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Ui {
        let inner_area = ui.available_rect_before_wrap() - self.inner_margin - self.outer_margin;
        let mut inner_ui = ui.new_child(UiBuilder::new().max_rect(inner_area));

        content(&mut inner_ui);

        inner_ui
    }

    fn paint(&mut self, ui: &mut Ui, where_to_paint: ShapeIdx, area: Rect) {
        let decoration = self.decoration.take().unwrap_or(FrameDecoration::None); // instead of
        // resuming, return
        let shape = decoration.into_shape(ui, area);

        let painter = ui.painter();

        painter.set(where_to_paint, shape);
    }
}
