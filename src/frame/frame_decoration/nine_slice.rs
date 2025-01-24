use crate::{
    nine_slice::NineSliceCache,
    widget_state::{WidgetState, WidgetStateType},
};
use egui::{Color32, Rect, Shape, TextureId, Ui};

pub struct FrameDecorationNineSlice<'a> {
    pub texture_file_path: &'a str,
    pub tint: Color32,
}

impl FrameDecorationNineSlice<'_> {
    pub fn compute_uvs_and_destinations(
        &self,
        ui: &Ui,
        area: Rect,
    ) -> (Vec<Rect>, Vec<Rect>, TextureId) {
        let cache = NineSliceCache::from_texture(
            ui,
            self.texture_file_path,
            area,
            ui.available_size_before_wrap(),
        );

        (
            cache.uvs.clone(),
            cache.destinations.clone(),
            cache.texture.id(),
        )
    }
}

pub(super) fn make_nine_slice_decoration_shape(
    ui: &Ui,
    decoration: FrameDecorationNineSlice,
    area: Rect,
) -> Shape {
    let cache = NineSliceCache::load_or_new(ui, None, WidgetStateType::Runtime, || {
        NineSliceCache::from_texture(
            ui,
            decoration.texture_file_path,
            area,
            ui.available_size_before_wrap(),
        )
    });

    let ui_size = ui.available_size_before_wrap();

    if cache.last_ui_size == ui_size {
        if let Some(shape) = cache.shape {
            println!("used shape from cache");

            return shape;
        }
    }

    let mut cache = NineSliceCache::from_texture(
            ui,
            decoration.texture_file_path,
            area,
            ui.available_size_before_wrap(),
        );

    let shape = create_shape(
        cache.uvs.clone(),
        cache.destinations.clone(),
        cache.texture.id(),
    );

    cache.shape = Some(shape.clone());

    cache.last_ui_size = ui_size;

    cache.save_state(ui, None, WidgetStateType::Runtime);

    println!("recreated shape and cached");

    shape
}

fn create_shape(uvs: Vec<Rect>, destinations: Vec<Rect>, texture: TextureId) -> Shape {
    let mut shapes: Vec<Shape> = vec![];

    for (uv, destination) in uvs.iter().zip(destinations) {
        let shape = Shape::image(texture, destination, *uv, Color32::WHITE);

        shapes.push(shape);
    }

    Shape::Vec(shapes)
}
