use egui::{epaint::RectShape, Color32, Rect, Rounding, Shape, Stroke};

pub struct FrameDecorationSimple {
    background_color: Color32,
    stroke: Stroke,
    rounding: Rounding,
}

pub(super) fn make_simple_decoration_shape(area: Rect, decoration: FrameDecorationSimple) -> Shape {
    Shape::Rect(RectShape::new(area, decoration.rounding, decoration.background_color, decoration.stroke))
}
