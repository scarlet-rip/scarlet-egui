pub(super) mod nine_slice;
pub(super) mod simple;

use egui::{Rect, Shape, Ui};
use nine_slice::{make_nine_slice_decoration_shape, FrameDecorationNineSlice};
use simple::{make_simple_decoration_shape, FrameDecorationSimple};

pub enum FrameDecoration<'a> {
    None,
    Simple(FrameDecorationSimple),
    NineSlice(FrameDecorationNineSlice<'a>),
}

impl FrameDecoration<'_> {
    pub fn into_shape(self, ui: &Ui, area: Rect) -> Shape {
        match self {
            FrameDecoration::None => Shape::Noop,
            FrameDecoration::Simple(decoration) => make_simple_decoration_shape(area, decoration),
            FrameDecoration::NineSlice(decoration) => {
                make_nine_slice_decoration_shape(ui, decoration, area)
            }
        }
    }
}
