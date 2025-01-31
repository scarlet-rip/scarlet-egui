use crate::{
    nine_slice::NineSliceCache,
    widget_state::{WidgetState, WidgetStateType},
};
use egui::{Color32, Rect, Shape, TextureId, Ui, load::SizedTexture};

pub struct FrameDecorationNineSlice {
    pub texture: SizedTexture,
    pub tint: Option<Color32>,
}

impl FrameDecorationNineSlice {
    pub fn compute_uvs_and_destinations(
        &self,
        ui: &Ui,
        area: Rect,
    ) -> (Vec<Rect>, Vec<Rect>, TextureId) {
        let cache = NineSliceCache::from_texture(
            self.texture,
            area,
            ui.available_size_before_wrap(),
        );

        (
            cache.uvs.clone(),
            cache.destinations.clone(),
            cache.texture.id,
        )
    }
}

pub(super) fn make_nine_slice_decoration_shape(
    ui: &Ui,
    decoration: FrameDecorationNineSlice,
    area: Rect,
    id_salt: &str,
) -> Shape {
    let available_ui_size = ui.available_size_before_wrap();

    let cache = NineSliceCache::load_or_new(ui, Some(id_salt), WidgetStateType::Runtime, || {
        NineSliceCache::from_texture(decoration.texture, area, available_ui_size)
    });

    if cache.last_available_ui_size == available_ui_size {
        if let Some(shape) = cache.shape {
            return shape;
        }
    }

    let mut cache = NineSliceCache::from_texture(
        decoration.texture,
        area,
        ui.available_size_before_wrap(),
    );

    let shape = create_shape(
        cache.uvs.clone(),
        cache.destinations.clone(),
        cache.texture.id,
        decoration.tint,
    );

    cache.shape = Some(shape.clone());

    cache.last_available_ui_size = available_ui_size;

    cache.save_state(ui, Some(id_salt), WidgetStateType::Runtime);

    shape
}

fn create_shape(
    uvs: Vec<Rect>,
    destinations: Vec<Rect>,
    texture: TextureId,
    tint: Option<Color32>,
) -> Shape {
    let mut shapes: Vec<Shape> = vec![];

    for (uv, destination) in uvs.iter().zip(destinations) {
        let shape = Shape::image(texture, destination, *uv, tint.unwrap_or(Color32::WHITE));

        shapes.push(shape);
    }

    Shape::Vec(shapes)
}
